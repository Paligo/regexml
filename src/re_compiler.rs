use std::rc::Rc;

use ahash::{HashSet, HashSetExt};

use crate::{
    character_class::{CharacterClass, PredicateFn},
    op_atom::Atom,
    op_back_reference::BackReference,
    op_bol::Bol,
    op_capture::Capture,
    op_character_class::CharClass,
    op_choice::Choice,
    op_end_program::EndProgram,
    op_eol::Eol,
    op_greedy_fixed::GreedyFixed,
    op_nothing::Nothing,
    op_reluctant_fixed::ReluctantFixed,
    op_repeat::Repeat,
    op_sequence::Sequence,
    operation::{Operation, OperationControl, MATCHES_ZLS_ANYWHERE},
    re_flags::ReFlags,
    re_program::{ReProgram, OPT_HASBACKREFS},
};

// No flags (nothing special)
pub(crate) const NODE_NORMAL: u32 = 0;
// True if top level expr
pub(crate) const NODE_TOPLEVEL: u32 = 2;

pub(crate) struct ReCompiler {
    // Input state for compiling regular expression

    // input string
    pattern: Vec<char>,
    // length of the pattern string
    len: usize,
    // current input index into ac
    idx: usize,
    // total number of paren pairs
    capturing_open_paren_count: usize,

    // {m, n} stacks
    // minimum number of matches
    bracket_min: usize,
    // maximum number of matches
    bracket_max: usize,

    is_xpath: bool,
    is_xpath_30: bool,
    is_xsd_11: bool,

    captures: HashSet<usize>,
    has_back_references: bool,

    re_flags: ReFlags,

    warning: Vec<String>,
}

#[derive(Debug)]
pub enum Error {
    Internal,
    Syntax(String),
}

impl Error {
    pub(crate) fn syntax(s: impl Into<String>) -> Error {
        Error::Syntax(s.into())
    }
}

enum CharacterClassOrBackReference {
    CharacterClass(CharacterClass),
    BackReference(usize),
}

impl From<CharacterClass> for CharacterClassOrBackReference {
    fn from(cc: CharacterClass) -> Self {
        Self::CharacterClass(cc)
    }
}

impl ReCompiler {
    pub(crate) fn new(re_flags: ReFlags) -> Self {
        Self {
            pattern: Vec::new(),
            len: 0,
            idx: 0,
            capturing_open_paren_count: 0,
            bracket_min: 0,
            bracket_max: 0,
            is_xpath: true,
            is_xpath_30: true,
            is_xsd_11: false,
            captures: HashSet::new(),
            has_back_references: false,
            re_flags,
            warning: Vec::new(),
        }
    }

    fn bracket(&mut self) -> Result<(), Error> {
        if self.idx >= self.len {
            return Err(Error::Internal);
        }
        if self.pattern[self.idx] != '{' {
            return Err(Error::Internal);
        }
        self.idx += 1;

        // next char must be a digit
        if self.idx >= self.len || !self.pattern[self.idx].is_ascii_digit() {
            return Err(Error::syntax("Expected digit"));
        }

        // get min ('m' of {m,n}) number
        let mut number = String::new();
        while self.idx < self.len && self.pattern[self.idx].is_ascii_digit() {
            number.push(self.pattern[self.idx]);
            self.idx += 1;
        }

        self.bracket_min = number
            .parse::<usize>()
            .map_err(|_| Error::syntax("Expected valid number"))?;

        // if out of input, fail
        if self.idx >= self.len {
            return Err(Error::syntax("Expected comma or right bracket"));
        }

        // if end of expr, optional limit is 0
        if self.pattern[self.idx] == '}' {
            self.idx += 1;
            self.bracket_max = self.bracket_min;
            return Ok(());
        }

        // must have at least {m,} and maybe {m,n}
        if self.idx >= self.len || self.pattern[self.idx] != ',' {
            return Err(Error::syntax("Expected comma"));
        }
        self.idx += 1;

        // if out of input, fail
        if self.idx >= self.len {
            return Err(Error::syntax("Expected comma or right bracket"));
        }

        // if {m,} max is unlimited
        if self.pattern[self.idx] == '}' {
            self.idx += 1;
            self.bracket_max = usize::MAX;
            return Ok(());
        }

        // next char must be a digit
        if self.idx >= self.len || !self.pattern[self.idx].is_ascii_digit() {
            return Err(Error::syntax("Unexpected digit"));
        }

        // get max number
        let mut number = String::new();
        while self.idx < self.len && self.pattern[self.idx].is_ascii_digit() {
            number.push(self.pattern[self.idx]);
            self.idx += 1;
        }

        self.bracket_max = number
            .parse::<usize>()
            .map_err(|_| Error::syntax("Expected valid number"))?;

        // optional repetitions must be >= 0
        if self.bracket_max < self.bracket_min {
            return Err(Error::syntax("Bad range"));
        }

        // must have close brace
        if self.idx >= self.len || self.pattern[self.idx] != '}' {
            return Err(Error::syntax("Missing closing brace"));
        }
        Ok(())
    }

    fn escape(&mut self, in_square_brackets: bool) -> Result<CharacterClassOrBackReference, Error> {
        // "Shouldn't" happen
        if self.pattern[self.idx] != '\\' {
            return Err(Error::Internal);
        }

        // escape shouldn't occur as last character in string!
        if self.idx + 1 >= self.len {
            return Err(Error::syntax("Escape terminates string"));
        }

        // switch on character after backslash
        self.idx += 2;
        let escape_char = self.pattern[self.idx - 1];
        match escape_char {
            'n' => Ok(CharacterClass::Char('\n').into()),
            'r' => Ok(CharacterClass::Char('\r').into()),
            't' => Ok(CharacterClass::Char('\t').into()),
            '\\' | '|' | '.' | '-' | '^' | '?' | '*' | '+' | '{' | '}' | '(' | ')' | '[' | ']' => {
                Ok(CharacterClass::Char(escape_char).into())
            }
            '$' => {
                if self.is_xpath {
                    Ok(CharacterClass::Char('$').into())
                } else {
                    Err(Error::syntax("In XSD, '$' must not be escaped"))
                }
            }
            's' => Ok(CharacterClass::escape_s_lower().into()),
            'S' => Ok(CharacterClass::escape_s_upper().into()),
            // TODO: i, I, c, C, d, D, w, W
            'p' | 'P' => {
                if self.idx == self.len {
                    return Err(Error::syntax(format!(
                        "Expected '{{' after \\{}",
                        escape_char
                    )));
                }
                if self.pattern[self.idx] != '{' {
                    return Err(Error::syntax(format!(
                        "Expected '{{' after \\{}",
                        escape_char
                    )));
                }
                let from = self.idx + 1;
                let close = self
                    .pattern
                    .iter()
                    .skip(from)
                    .position(|c| *c == '}')
                    .ok_or(Error::syntax(format!(
                        "No closing '}}' after \\{}",
                        escape_char
                    )))?;
                let block = &self.pattern[self.idx..close];
                todo!()
            }
            '0' => Err(Error::syntax("Octal escapes are not allowed")),
            '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                if in_square_brackets {
                    return Err(Error::syntax(
                        "Backreferences not allowed within character classes",
                    ));
                }
                if !self.is_xpath {
                    return Err(Error::syntax("digit not allowed after \\"));
                }
                let mut back_ref = (escape_char as usize) - ('0' as usize);
                while self.idx < self.len {
                    let c1 = self.pattern[self.idx].to_digit(10);
                    if let Some(c1) = c1 {
                        let back_ref2 = back_ref * 10 + (c1 as usize);
                        // TODO shaky conversion
                        if back_ref2 > (self.capturing_open_paren_count - 1) {
                            break;
                        }
                        back_ref = back_ref2;
                        self.idx += 1;
                    } else {
                        break;
                    }
                }
                if !self.captures.contains(&back_ref) {
                    let explanation = if back_ref > (self.capturing_open_paren_count - 1) {
                        "(no such group)"
                    } else {
                        "(group not yet closed)"
                    };
                    return Err(Error::syntax(format!(
                        "invalid backreference \\{} {}",
                        back_ref, explanation
                    )));
                }
                self.has_back_references = true;
                // for convenience a back-reference is treated as a character
                // class, though this is a fiction
                // TODO: need to probably introduce CharacterClass::BackReference
                // so we can detect these. Alternatively return an enum that's
                // either a back reference or a character class here.
                Ok(CharacterClassOrBackReference::BackReference(back_ref))
            }
            escape_char => Err(Error::syntax(format!(
                "Escape character '{}' not allowed",
                escape_char
            ))),
        }
    }

    fn parse_character_class(&mut self) -> Result<CharacterClass, Error> {
        // check for bac calling or empty class
        if self.pattern[self.idx] != '[' {
            return Err(Error::Internal);
        }

        // check for unterminated or empty class
        let index = self.idx;
        self.idx += 1;
        if self.idx + 1 >= self.len || self.pattern[self.idx] == ']' {
            return Err(Error::syntax("Missing ']"));
        }

        // parse class declaration
        let mut simple_char = None;
        let mut positive = true;
        let mut defining_range = false;
        let mut range_start = None;
        let mut range_end = None;

        let mut range = HashSet::new();
        let mut addend: Option<CharacterClass> = None;
        let mut subtrahend = None;

        if self.there_follows("^") {
            if self.there_follows("&-[") {
                return Err(Error::syntax("Nothing before subtraction operator"));
            } else if self.there_follows("^]") {
                return Err(Error::syntax("Empty negative character group"));
            } else {
                positive = false;
                self.idx += 1;
            }
        } else if self.there_follows("-[") {
            return Err(Error::syntax("Nothing before subtraction operator"));
        }

        while self.idx < self.len && self.pattern[self.idx] != ']' {
            let ch = self.pattern[self.idx];
            simple_char = None;
            match ch {
                '[' => {
                    return Err(Error::syntax("Unescaped '[' within square brackets"));
                }
                '\\' => {
                    // escape always advances the stream
                    let cc = self.escape(true)?;
                    match cc {
                        CharacterClassOrBackReference::CharacterClass(CharacterClass::Char(c)) => {
                            simple_char = Some(c);
                        }
                        CharacterClassOrBackReference::CharacterClass(cc) => {
                            if defining_range {
                                return Err(Error::syntax(
                                    "Multi-character escape cannot follow '-'",
                                ));
                            } else if let Some(a) = addend {
                                addend = Some(a.union(cc));
                            } else {
                                addend = Some(cc);
                            }
                            continue;
                        }
                        _ => unreachable!(),
                    }
                }
                '-' => {
                    if self.there_follows("-[") {
                        self.idx += 1;
                        subtrahend = Some(self.parse_character_class()?);
                        if !self.there_follows("]") {
                            return Err(Error::syntax("Expected closing ']' after subtraction"));
                        } else if self.there_follows("-]") {
                            simple_char = Some('-');
                            self.idx += 1;
                            continue;
                        } else if range_start.is_some() {
                            defining_range = true;
                            self.idx += 1;
                            continue;
                        }
                        if self.there_follows("--") && !self.there_follows("--[") {
                            return Err(Error::syntax("Unescaped hyphen at start of range"));
                        } else if !self.is_xsd_11
                            && self.pattern[self.idx - 1] != '['
                            && self.pattern[self.idx - 1] != '^'
                            && !self.there_follows("]")
                            && !self.there_follows("-[")
                        {
                            return Err(Error::syntax("In XSD 1.0, hyphen is allowed only at the beginning or end of a positive character group"));
                        } else {
                            simple_char = Some('-');
                            self.idx += 1;
                        }
                    }
                }
                _ => {
                    simple_char = Some(ch);
                    self.idx += 1;
                }
            }

            // handle simple character simpleChar
            if defining_range {
                // if we are defining a range make it now
                range_end = simple_char;

                // actually create a range if the range is ok
                if let (Some(start), Some(end)) = (range_start, range_end) {
                    if start > end {
                        return Err(Error::syntax("Bad character range: start > end"));
                        // Technically this is not an error in
                        // XSD, merely a no-op; but it is so
                        // utterly pointless that it is almost certainly a mistake; and we have no
                        // way of indicating warnings.
                    }
                    for c in start..=end {
                        range.insert(c);
                    }
                    if self.re_flags.is_case_independent() {
                        if start == 'a' && end == 'z' {
                            for c in 'A'..='Z' {
                                range.insert(c);
                            }
                            // TODO
                            // for (int v = 0; v < CaseVariants.ROMAN_VARIANTS.length; v++) {
                            //     range.add(CaseVariants.ROMAN_VARIANTS[v]);
                            // }
                        } else if start == 'A' && end == 'Z' {
                            for c in 'a'..='z' {
                                range.insert(c);
                            }
                            // TODO
                            // for (int v = 0; v < CaseVariants.ROMAN_VARIANTS.length; v++) {
                            //     range.add(CaseVariants.ROMAN_VARIANTS[v]);
                            // }
                        } else {
                            for _ in start..=end {
                                // TODO
                                // int[] variants = CaseVariants.getCaseVariants(k);
                                // for (int variant : variants) {
                                //     range.add(variant);
                                // }
                            }
                        }
                    }
                    // we are don defining the range
                    defining_range = false;
                    range_start = None;
                }
            } else {
                let simple_char = simple_char.unwrap();
                // if simple character and not start of range, include it (see XSD 1.1 rules)
                if self.there_follows("-") {
                    if self.there_follows("-[")
                        || self.there_follows("-]")
                        || self.there_follows("--[")
                    {
                        range.insert(simple_char);
                    } else if self.there_follows("--") {
                        return Err(Error::syntax("Unescaped hyphen cannot act as end of range"));
                    } else {
                        range_start = Some(simple_char);
                    }
                } else {
                    range.insert(simple_char);
                    if self.re_flags.is_case_independent() {
                        // TODO
                        // int[] variants = CaseVariants.getCaseVariants(simpleChar);
                        // for (int variant : variants) {
                        //     range.add(variant);
                        // }
                    }
                }
            }
        }

        // shouldn't be out of input
        if self.idx == self.len {
            return Err(Error::syntax("Unterminated character class"));
        }

        // absorb the ']' end of class marker
        self.idx += 1;
        let mut result = CharacterClass::CharSet(range);
        if let Some(addend) = addend {
            result = result.union(addend);
        }
        if !positive {
            result = result.complement();
        }
        if let Some(subtrahend) = subtrahend {
            result = result.difference(subtrahend);
        }
        Ok(result)
    }

    fn parse_atom(&mut self) -> Result<Operation, Error> {
        // length of atom
        let mut len_atom = 0;

        // loop while we've got input
        let mut ub = Vec::new();

        while self.idx < self.len {
            // is there a next char?
            if (self.idx + 1) < self.len {
                let mut c = self.pattern[self.idx + 1];

                // if the next 'char' is an escape, look past the whole escape
                if self.pattern[self.idx] == '\\' {
                    let idx_escape = self.idx;

                    self.escape(false)?;
                    if self.idx < self.len {
                        c = self.pattern[self.idx];
                    }
                    self.idx = idx_escape;
                }

                // switch on next har
                if matches!(c, '{' | '?' | '*' | '+') {
                    // If the next character is a quantifier operator and
                    // our atom is non-empty, the current character should
                    // bind to the quantifier operator rather than the atom
                    if len_atom > 0 {
                        break;
                    }
                }
            }
            match self.pattern[self.idx] {
                ']' | '.' | '[' | '(' | ')' | '|' => {
                    break;
                }
                '{' | '?' | '*' | '+' => {
                    // we should have an atom by now
                    if len_atom == 0 {
                        return Err(Error::syntax("Missing expression before quantifier"));
                    }
                    break;
                }
                '}' => {
                    return Err(Error::syntax("Unescaped right curly brace"));
                }
                '\\' => {
                    // get the escaped character (advanced input automatically)
                    let idx_before_escape = self.idx;
                    let character_class = self.escape(false)?;

                    // check if it's a simple escape (as opposed to, say, a backreference)
                    if let CharacterClassOrBackReference::CharacterClass(CharacterClass::Char(ch)) =
                        character_class
                    {
                        ub.push(ch);
                        len_atom += 1;
                    } else {
                        // not a simple escape, so backup to where we were before the escape
                        self.idx = idx_before_escape;
                        break;
                    }
                }
                c => {
                    if (c == '^' || c == '$') && self.is_xpath {
                        break;
                    }
                    ub.push(self.pattern[self.idx]);
                    self.idx += 1;
                    len_atom += 1;
                }
            }
        }
        // this shouldn't happen
        if ub.is_empty() {
            return Err(Error::Internal);
        }

        // return the instruction
        Ok(Operation::from(Atom::new(ub)))
    }

    fn parse_terminal(&mut self, flags: &[u32]) -> Result<Operation, Error> {
        match self.pattern[self.idx] {
            '$' => {
                if self.is_xpath {
                    self.idx += 1;
                    return Ok(Operation::from(Eol));
                }
            }
            '^' => {
                if self.is_xpath {
                    self.idx += 1;
                    return Ok(Operation::from(Bol));
                }
            }
            '.' => {
                self.idx += 1;
                let predicate = if self.re_flags.is_single_line() {
                    // in XPath with the 's' flag, '.' matches everything
                    |_| true
                } else {
                    // Don't we have enough information to create a non-predicate
                    // character class?
                    // in XSD, "." matches everything except \n and \r
                    |c| c != '\n' && c != '\r'
                };
                return Ok(Operation::from(CharClass::new(CharacterClass::Predicate(
                    PredicateFn::new(predicate),
                ))));
            }
            '[' => {
                let range = self.parse_character_class()?;
                return Ok(Operation::from(CharClass::new(range)));
            }
            '(' => return self.parse_expr(flags),
            ')' => return Err(Error::syntax("Unescaped closing ')'")),
            '|' => return Err(Error::Internal),
            ']' => return Err(Error::syntax("Unexpected closing ']'")),
            '?' | '+' | '{' | '*' => {
                return Err(Error::syntax("No expression before quantifier"));
            }
            '\\' => {
                // don't forget, escape() advances the input stream!
                let idx_before_escape = self.idx;
                let esc = self.escape(false)?;

                match esc {
                    CharacterClassOrBackReference::BackReference(back_ref) => {
                        if self.capturing_open_paren_count <= back_ref {
                            return Err(Error::syntax("Bad backreference"));
                        }
                        return Ok(Operation::from(BackReference::new(back_ref)));
                    }
                    CharacterClassOrBackReference::CharacterClass(CharacterClass::Char(c)) => {
                        // we had a simple escape and we want to have it end up in an atom,
                        // so we back up and fall through to the default handling
                        self.idx = idx_before_escape;
                    }
                    CharacterClassOrBackReference::CharacterClass(character_class) => {
                        return Ok(Operation::from(CharClass::new(character_class)))
                    }
                }
            }
            _ => {}
        }
        // if it wasn't one of the above, it must be the start of an atom.
        self.parse_atom()
    }

    fn piece(&mut self, flags: &[u32]) -> Result<Operation, Error> {
        // values to pass by refrence to terminal()
        let terminal_flags = vec![NODE_NORMAL];

        // get terminal symbol
        let ret = self.parse_terminal(&terminal_flags)?;

        // or in flags from terminal symnbol
        let mut modified_flags = flags.to_vec();
        modified_flags[0] = flags[0] | terminal_flags[0];

        // advance input, set NODE_NULLABLE flag and do santify checks
        if self.idx >= self.len {
            return Ok(ret);
        }

        let quantifier_type = self.pattern[self.idx];

        match quantifier_type {
            '?' | '*' | '+' => {
                // eat quantifier character
                self.idx += 1;
            }
            '{' => self.bracket()?,
            _ => {}
        }

        let mut quantifier_type = Some(quantifier_type);

        match ret {
            Operation::Bol(_) | Operation::Eol(_) => {
                // pretty meaningless but legal. If the quantifier allows zero
                // occurrences, ignore the instruction. Otherwise, ignore the
                // quantifier.
                if quantifier_type == Some('?')
                    || quantifier_type == Some('*')
                    || (quantifier_type == Some('{') && self.bracket_min == 0)
                {
                    return Ok(Operation::from(Nothing));
                } else {
                    quantifier_type = None
                }
                if ret.matches_empty_string() == MATCHES_ZLS_ANYWHERE {
                    match quantifier_type {
                        Some('?') => {
                            // can ignore the quantifier
                            quantifier_type = None
                        }
                        Some('+') => {
                            // '*' and '+' are equivalent
                            quantifier_type = Some('*');
                        }
                        Some('{') => {
                            // bounds are meaningless
                            quantifier_type = Some('*')
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }

        let mut greedy = true;

        // if the next character is a '?', make the quantifier non-greedy (reluctant)
        if self.idx < self.len && self.pattern[self.idx] == '?' {
            if !self.is_xpath {
                return Err(Error::syntax("Reluctant quantifier not allowed in XSD"));
            }
            self.idx += 1;
            greedy = false;
        }
        let mut min = 1;
        let mut max = 1;

        match quantifier_type {
            Some('{') => {
                min = self.bracket_min;
                max = self.bracket_max;
            }
            Some('?') => {
                min = 0;
                max = 1;
            }
            Some('+') => {
                min = 1;
                max = usize::MAX;
            }
            Some('*') => {
                min = 0;
                max = usize::MAX;
            }
            _ => {}
        }

        if max == 0 {
            Ok(Operation::from(Nothing))
        } else if min == 1 && max == 1 {
            Ok(ret)
        } else if greedy {
            // actually do the quantifier now
            if let Some(match_length) = ret.get_match_length() {
                Ok(Operation::from(GreedyFixed::new(
                    Rc::new(ret),
                    min,
                    max,
                    match_length,
                )))
            } else {
                Ok(Operation::from(Repeat::new(Rc::new(ret), min, max, true)))
            }
        } else if let Some(match_length) = ret.get_match_length() {
            Ok(Operation::from(ReluctantFixed::new(
                Rc::new(ret),
                min,
                max,
                match_length,
            )))
        } else {
            Ok(Operation::from(Repeat::new(Rc::new(ret), min, max, false)))
        }
    }

    fn parse_branch(&mut self) -> Result<Operation, Error> {
        // get each possibly quantified piece and concat
        let mut current = None;
        let mut quantifier_flags = vec![1];
        while self.idx < self.len && self.pattern[self.idx] != '|' && self.pattern[self.idx] != ')'
        {
            // get new node
            quantifier_flags[0] = NODE_NORMAL;
            let op = self.piece(&quantifier_flags)?;
            if let Some(c) = current {
                current = Some(Self::make_sequence(c, op));
            } else {
                current = Some(op);
            }
        }
        if let Some(current) = current {
            Ok(current)
        } else {
            // if we don't run loop, make a nothing node
            Ok(Operation::from(Nothing))
        }
    }

    fn parse_expr(&mut self, compiler_flags: &[u32]) -> Result<Operation, Error> {
        // create open paren node unless we were called from the top level (which has no parens)
        let mut paren = None;
        let mut group = 0;
        let mut branches = Vec::new();
        let close_parens = self.capturing_open_paren_count;
        let mut capturing = true;
        if (compiler_flags[0] & NODE_TOPLEVEL) == 0 && self.pattern[self.idx] == '(' {
            // if it's a cluster (rather than a proper subexpression ie with backrefs)
            if (self.idx + 2) < self.len
                && self.pattern[self.idx + 1] == '?'
                && self.pattern[self.idx + 2] == ':'
            {
                if !self.is_xpath_30 {
                    return Err(Error::syntax(
                        "Non-capturing groups only allowed in XPath 3.0",
                    ));
                }
                paren = Some(2);
                self.idx += 3;
                capturing = false;
            } else {
                paren = Some(1);
                self.idx += 1;
                group = self.capturing_open_paren_count;
                self.capturing_open_paren_count += 1;
            }
        }
        let mut compiler_flags = compiler_flags.to_vec();
        compiler_flags[0] &= !NODE_TOPLEVEL;

        // process contents of first branch node
        branches.push(self.parse_branch()?);
        // loop through brnaches
        while self.idx < self.len && self.pattern[self.idx] == '|' {
            self.idx += 1;
            branches.push(self.parse_branch()?);
        }

        let mut op = if branches.len() == 1 {
            branches.remove(0)
        } else {
            Operation::from(Choice::new(branches.into_iter().map(Rc::new).collect()))
        };

        // create an ending node (either a close paren or an OP_END)
        if paren.is_some() {
            if self.idx < self.len && self.pattern[self.idx] == ')' {
                self.idx += 1;
            } else {
                return Err(Error::syntax("Missing close paren"));
            }
            if capturing {
                op = Operation::from(Capture::new(group, Rc::new(op)));
                self.captures.insert(close_parens);
            }
        } else {
            op = Self::make_sequence(op, Operation::from(EndProgram));
        }
        Ok(op)
    }

    fn there_follows(&self, s: &str) -> bool {
        let chars = s.chars().collect::<Vec<_>>();

        if (self.idx + chars.len()) > self.len {
            return false;
        }
        for c in chars {
            if self.pattern[self.idx + 1] != c {
                return false;
            }
        }
        true
    }

    fn make_sequence(o1: Operation, o2: Operation) -> Operation {
        match (o1, o2) {
            (Operation::Sequence(o1), Operation::Sequence(o2)) => {
                let list1 = o1.operations;
                let list2 = o2.operations;
                let mut list = Vec::with_capacity(list1.len() + list2.len());
                list.extend(list1);
                list.extend(list2);
                Operation::Sequence(Sequence::new(list))
            }
            (Operation::Sequence(o1), o2) => {
                let mut list = o1.operations.clone();
                list.push(Rc::new(o2));
                Operation::Sequence(Sequence::new(list))
            }
            (o1, Operation::Sequence(o2)) => {
                let mut list = Vec::with_capacity(o2.operations.len() + 1);
                list.push(Rc::new(o1));
                list.extend(o2.operations);
                Operation::Sequence(Sequence::new(list))
            }
            (o1, o2) => {
                let list = vec![Rc::new(o1), Rc::new(o2)];
                Operation::Sequence(Sequence::new(list))
            }
        }
    }

    pub(crate) fn compile(&mut self, pattern: Vec<char>) -> Result<ReProgram, Error> {
        // initialize variables for compilation

        // save pattern in instance variable
        self.pattern = pattern;
        // precompute pattern length for speed
        self.len = self.pattern.len();
        // set parsing index to the first character
        self.idx = 0;
        // set paren level to 1 (the implicit outer parens)
        self.capturing_open_paren_count = 1;

        if self.re_flags.is_literal() {
            // 'q' flag is set
            // create a string node
            let ret = Operation::from(Atom::new(self.pattern.clone()));
            let end_node = Operation::from(EndProgram);
            let seq = Self::make_sequence(ret, end_node);
            Ok(ReProgram::new(
                Rc::new(seq),
                Some(self.capturing_open_paren_count),
                self.re_flags.clone(),
            ))
        } else {
            if self.re_flags.is_allow_whitespace() {
                // 'x' flag is set. preprocess the expression to strip whitespace,
                // other than between square brackets
                let mut sb = Vec::new();
                let mut nesting = 0;
                let mut escaped = false;
                for ch in self.pattern.iter() {
                    match ch {
                        '\\' if !escaped => {
                            escaped = true;
                            sb.push(*ch);
                        }
                        '[' if !escaped => {
                            nesting += 1;
                            sb.push(*ch);
                        }
                        ']' if !escaped => {
                            nesting -= 1;
                            sb.push(*ch);
                        }
                        _ => {
                            // TODO: wrong whitespace
                            if nesting == 0 && ch.is_ascii_whitespace() {
                                // no action
                            } else {
                                escaped = false;
                                sb.push(*ch);
                            }
                        }
                    }
                }
                self.pattern = sb;
                self.len = self.pattern.len();
            }

            // initialize pass by reference flags value
            let compiler_flags = vec![NODE_TOPLEVEL];

            // parse expression
            let exp = self.parse_expr(&compiler_flags)?;

            // should be at end of input
            if self.idx != self.len {
                if self.pattern[self.idx] == ')' {
                    return Err(Error::syntax("Unmatched close paren"));
                }
                return Err(Error::syntax("Unexpected input remains"));
            }

            let mut program = ReProgram::new(
                Rc::new(exp),
                Some(self.capturing_open_paren_count),
                self.re_flags.clone(),
            );
            if self.has_back_references {
                program.optimization_flags |= OPT_HASBACKREFS;
            }
            Ok(program)
        }
    }
}

#[cfg(test)]
mod tests {
    use insta::assert_debug_snapshot;

    use super::*;

    fn compiled(pattern: &str) -> ReProgram {
        let re_flags = ReFlags::new("", "XP30").unwrap();
        let mut re_compiler = ReCompiler::new(re_flags);
        let pattern = pattern.chars().collect();
        re_compiler.compile(pattern).unwrap()
    }

    #[test]
    fn test_simple_compile() {
        assert_debug_snapshot!(compiled("abc").operation);
    }

    #[test]
    fn test_compile_a_star() {
        assert_debug_snapshot!(compiled("a*").operation);
    }

    #[test]
    fn test_compile_combined() {
        assert_debug_snapshot!(compiled("^a?b+c*$").operation);
    }
}
