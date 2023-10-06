use ahash::{HashSet, HashSetExt};

use crate::{
    character_class::CharacterClass, op_atom::OpAtom, op_back_reference::OpBackReference,
    op_bol::OpBol, op_character_class::OpCharacterClass, op_eol::OpEol, operation::Operation,
    re_program::ReFlags,
};

// No flags (nothing special)
pub(crate) const NODE_NORMAL: u32 = 0;
// True if top level expr
pub(crate) const NODE_TOPLEVEL: u32 = 2;

struct ReCompiler<'a> {
    // Input state for compiling regular expression

    // input string
    pattern: &'a [char],
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

enum Error {
    Internal,
    Syntax(String),
}

impl Error {
    fn syntax(s: impl Into<String>) -> Error {
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

impl<'a> ReCompiler<'a> {
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
        Ok(Operation::from(OpAtom::new(ub)))
    }

    fn parse_terminal(&mut self, flags: Vec<u32>) -> Result<Operation, Error> {
        match self.pattern[self.idx] {
            '$' => {
                if self.is_xpath {
                    self.idx += 1;
                    return Ok(Operation::from(OpEol));
                }
            }
            '^' => {
                if self.is_xpath {
                    self.idx += 1;
                    return Ok(Operation::from(OpBol));
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
                return Ok(Operation::from(OpCharacterClass::new(
                    CharacterClass::Predicate(Box::new(predicate)),
                )));
            }
            '[' => {
                let range = self.parse_character_class()?;
                return Ok(Operation::from(OpCharacterClass::new(range)));
            }
            '(' => return self.parse_expr(flags),
            ')' => return Err(Error::syntax("Unescaped closing ')'")),
            '|' => return Err(Error::Internal),
            ']' => return Err(Error::syntax("Unexpected closing ']'")),
            '0' => return Err(Error::syntax("Unexpected end of input")),
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
                        return Ok(Operation::from(OpBackReference::new(back_ref)));
                    }
                    CharacterClassOrBackReference::CharacterClass(CharacterClass::Char(c)) => {
                        // we had a simple escape and we want to have it end up in an atom,
                        // so we back up and fall through to the default handling
                        self.idx = idx_before_escape;
                    }
                    CharacterClassOrBackReference::CharacterClass(character_class) => {
                        return Ok(Operation::from(OpCharacterClass::new(character_class)))
                    }
                }
            }
            _ => {}
        }
        // if it wasn't one of the above, it must be the start of an atom.
        self.parse_atom()
    }

    fn parse_expr(&self, flags: Vec<u32>) -> Result<Operation, Error> {
        todo!()
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
}
