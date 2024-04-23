use std::cell::RefCell;

use icu_casemap::CaseMapper;

#[cfg(test)]
use crate::operation::Operation;
#[cfg(test)]
use std::rc::Rc;

use crate::{
    operation::OperationControl,
    re_compiler::Error,
    re_program::{ReProgram, OPT_HASBACKREFS, OPT_HASBOL},
};

#[derive(Debug)]
pub(crate) struct ReMatcher<'a> {
    // current program
    pub(crate) program: &'a ReProgram,
    // string being matched against
    pub(crate) search: Vec<char>,

    case_mapper: CaseMapper,
    // parenthesized subexpressions
    state: RefCell<State>,
}

#[derive(Debug)]
pub(crate) struct State {
    pub(crate) start_backref: Vec<Option<usize>>,
    pub(crate) end_backref: Vec<Option<usize>>,
    pub(crate) capture_state: CaptureState,
    pub(crate) anchored_match: bool,
}

impl State {
    pub(crate) fn new() -> Self {
        Self {
            start_backref: vec![],
            end_backref: vec![],
            capture_state: CaptureState::new(),
            anchored_match: false,
        }
    }
}

impl<'a> ReMatcher<'a> {
    pub(crate) fn new(program: &'a ReProgram, search: &str) -> Self {
        let search = search.chars().collect::<Vec<_>>();
        Self {
            program,
            search,
            state: RefCell::new(State::new()),
            case_mapper: CaseMapper::new(),
        }
    }

    pub(crate) fn match_at(&self, i: usize, anchored: bool) -> bool {
        // initialize start pointer, paren cache and paren count
        self.set_paren_count(1);
        self.state.borrow_mut().anchored_match = anchored;
        self.set_paren_start(0, i);

        // allocate backref arrays (unless optimizations indicate otherwise)
        if self.program.optimization_flags & OPT_HASBACKREFS != 0 {
            self.state.borrow_mut().start_backref = vec![None; self.program.max_parens.unwrap()];
            self.state.borrow_mut().end_backref = vec![None; self.program.max_parens.unwrap()];
        }

        // match against string
        let mut iter = self.program.operation.matches_iter(self, i);
        if let Some(idx) = iter.next() {
            self.set_paren_end(0, idx);
            true
        } else {
            // didn't match
            self.state.borrow_mut().capture_state.paren_count = 0;
            false
        }
    }

    pub(crate) fn matches(&mut self, i: usize) -> bool {
        // clear the captured group state
        self.state.borrow_mut().capture_state = CaptureState::new();

        // can we optimize the search by looking for new lines?
        if self.program.optimization_flags & OPT_HASBOL == OPT_HASBOL {
            // non multi-line matching with BOL: must match at '0' index
            if !self.program.flags.is_multi_line() {
                return i == 0 && self.check_preconditions(i) && self.match_at(i, false);
            }

            // multi-line matching with BOL: seek to next line
            if self.match_at(i, false) {
                return true;
            }
            // TODO: transliterated as close to the Java code as possible to
            // make sure it works correctly. But can be cleaned up.
            let mut nl: isize = i.try_into().unwrap();
            loop {
                nl = self
                    .search
                    .iter()
                    .enumerate()
                    .skip(nl.try_into().unwrap())
                    .find(|(_, c)| **c == '\n')
                    .map(|(i, _)| i.try_into().unwrap())
                    .unwrap_or(-1)
                    + 1;

                if nl >= self.search.len().try_into().unwrap() || nl <= 0 {
                    // "^" does not match a NL at the end of the string
                    return false;
                } else if self.match_at(nl.try_into().unwrap(), false) {
                    return true;
                }
            }
        }

        // is the string long enough to match?
        let actual_length = self.search.len() - i;
        if actual_length < self.program.minimum_length {
            return false;
        }

        // can we optimize the search by looking for a prefix string?
        if let Some(prefix) = &self.program.prefix {
            // prefixed-anchored matching is possible
            let prefix_length = prefix.len();
            let ignore_case = self.program.flags.is_case_independent();
            for j in i..self.search.len() + 1 - prefix_length {
                let mut prefix_ok = true;
                if ignore_case {
                    for k in 0..prefix_length {
                        if !self.equal_case_blind(self.search[k + j], prefix[k]) {
                            prefix_ok = false;
                            break;
                        }
                    }
                } else {
                    for k in 0..prefix_length {
                        if self.search[k + j] != prefix[k] {
                            prefix_ok = false;
                            break;
                        }
                    }
                }

                // see if the whole prefix string matched
                if prefix_ok {
                    // we matched the full prefix at first_char, so try it
                    if self.match_at(j, false) {
                        return true;
                    }
                }
            }
            false
        } else {
            // no prefix known; but the first character must match a predicate
            if let Some(inv_list) = self.program.initial_character_class() {
                for j in i..self.search.len() {
                    if inv_list.contains(self.search[j]) && self.match_at(j, false) {
                        return true;
                    }
                }
                return false;
            }
            // check the preconditions
            if !self.check_preconditions(i) {
                return false;
            }

            // unprefixed matching must try for a match at each character
            for j in i..(self.search.len() + 1) {
                // try a match at index i
                if self.match_at(j, false) {
                    return true;
                }
            }
            false
        }
    }

    pub(crate) fn is_match(&mut self) -> bool {
        self.matches(0)
    }

    fn check_preconditions(&self, start: usize) -> bool {
        for precondition in &self.program.preconditions {
            if let Some(fixed_position) = precondition.fixed_position {
                let match_ = precondition
                    .operation
                    .matches_iter(self, fixed_position)
                    .next();
                if match_.is_none() {
                    return false;
                }
            } else {
                let mut i = start;
                if i < precondition.min_position {
                    i = precondition.min_position;
                }
                let mut found = false;
                for j in i..self.search.len() {
                    if (precondition.fixed_position.is_none()
                        || precondition.fixed_position == Some(j))
                        && precondition
                            .operation
                            .matches_iter(self, j)
                            .next()
                            .is_some()
                    {
                        found = true;
                        break;
                    }
                }
                if !found {
                    return false;
                }
            }
        }
        true
    }

    pub(crate) fn replace(&mut self, replacement: &[char]) -> Result<Vec<char>, Error> {
        // string to return
        let mut result = Vec::new();

        // start at position 0 and search the whole string
        let mut pos = 0;
        let len = self.search.len();

        let mut first_match = true;
        let mut simple_replacement = false;

        // try a match at each position
        while pos < len && self.matches(pos) {
            // append chars from input string before match
            // TODO: what happens if this returns None as there is no paren start?
            if let Some(start) = self.get_paren_start(0) {
                result.extend(&self.search[pos..start]);
            }
            if first_match {
                simple_replacement = self.program.flags.is_literal();
                first_match = false;
            }

            if !simple_replacement {
                // process references to captured substrings
                let max_capture = self.program.max_parens.unwrap() - 1;
                simple_replacement = true;
                let mut i = 0;
                while i < replacement.len() {
                    let ch = replacement[i];
                    match ch {
                        '\\' => {
                            simple_replacement = false;
                            i += 1;
                            let index = i;
                            if index >= replacement.len() {
                                return Err(Error::InvalidReplacementString(
                                    "Invalid escape at end of replacement string".to_string(),
                                ));
                            }
                            let ch = replacement[index];
                            match ch {
                                '\\' | '$' => {
                                    result.push(ch);
                                }
                                _ => {
                                    return Err(Error::InvalidReplacementString(
                                        format!("Invalid escape '{}' in replacement string", ch)
                                            .to_string(),
                                    ))
                                }
                            }
                        }
                        '$' => {
                            simple_replacement = false;
                            i += 1;
                            let index = i;
                            if index >= replacement.len() {
                                return Err(Error::InvalidReplacementString(
                                    "Invalid escape at end of replacement string".to_string(),
                                ));
                            }
                            let ch = replacement[index];
                            if !ch.is_ascii_digit() {
                                return Err(Error::InvalidReplacementString(
                                    "$ in replacement string must be followed by a digit"
                                        .to_string(),
                                ));
                            }
                            let mut n = (ch as usize) - ('0' as usize);
                            if max_capture <= 9 {
                                if max_capture >= n {
                                    let captured = self.get_paren(n);
                                    if let Some(captured) = captured {
                                        result.extend(captured);
                                    }
                                }
                            } else {
                                loop {
                                    i += 1;
                                    if i >= replacement.len() {
                                        break;
                                    }
                                    let ch = replacement[i];
                                    if ch.is_ascii_digit() {
                                        let m = n * 10 + ((ch as usize) - ('0' as usize));
                                        if m > max_capture {
                                            i -= 1;
                                            break;
                                        } else {
                                            n = m;
                                        }
                                    } else {
                                        i -= 1;
                                        break;
                                    }
                                }
                                let captured = self.get_paren(n);
                                if let Some(captured) = captured {
                                    result.extend(captured);
                                }
                            }
                        }
                        _ => {
                            result.push(ch);
                        }
                    }
                    i += 1;
                }
            } else {
                // append substitution without processing backreferences
                result.extend(replacement);
            }

            // move forward, skipping past match
            let mut newpos = self.get_paren_end(0).unwrap();

            if newpos == pos {
                newpos += 1;
            }

            // try new position
            pos = newpos;
        }

        // if no matches were found, return the input unchanged
        if first_match {
            return Ok(self.search.to_vec());
        }

        // if there's remaining input, append it
        result.extend(self.search[pos..len].iter());

        // return the string buffer
        Ok(result)
    }

    pub(crate) fn is_new_line(&self, i: usize) -> bool {
        self.search[i] == '\n'
    }

    #[cfg(test)]
    pub(crate) fn operation_matches(&self, op: Rc<Operation>) -> Vec<String> {
        let start_position = 0;
        let positions = op.matches_iter(self, start_position);
        let ranges = positions.map(|e| start_position..e);
        ranges
            .map(|r| self.search[r].iter().collect::<String>())
            .collect()
    }

    pub(crate) fn equal_case_blind(&self, a: char, b: char) -> bool {
        if a == b {
            return true;
        }
        let lowercase_a = self.case_mapper.simple_lowercase(a);
        let lowercase_b = self.case_mapper.simple_lowercase(b);
        if lowercase_a == lowercase_b {
            return true;
        }
        false
    }

    // state related

    fn start_backref_len(&self) -> usize {
        self.state.borrow().start_backref.len()
    }

    pub(crate) fn start_backref(&self, i: usize) -> Option<usize> {
        self.state.borrow().start_backref[i]
    }

    pub(crate) fn set_start_backref(&self, i: usize, value: Option<usize>) {
        self.state.borrow_mut().start_backref[i] = value;
    }

    pub(crate) fn end_backref(&self, i: usize) -> Option<usize> {
        self.state.borrow().end_backref[i]
    }

    pub(crate) fn set_end_backref(&self, i: usize, value: Option<usize>) {
        self.state.borrow_mut().end_backref[i] = value;
    }

    pub(crate) fn anchored_match(&self) -> bool {
        self.state.borrow().anchored_match
    }

    // capture state related

    pub(crate) fn get_paren(&self, group_nr: usize) -> Option<&[char]> {
        if group_nr < self.paren_count() {
            if let (Some(start), Some(end)) =
                (self.get_paren_start(group_nr), self.get_paren_end(group_nr))
            {
                return Some(&self.search[start..end]);
            }
        }
        None
    }

    fn startn_len(&self) -> usize {
        self.state.borrow().capture_state.startn.len()
    }

    pub(crate) fn get_paren_start(&self, group_nr: usize) -> Option<usize> {
        if group_nr < self.state.borrow().capture_state.startn.len() {
            return self.state.borrow().capture_state.startn[group_nr];
        }
        None
    }

    /// Sets the start of the paren level
    /// which is the paren level, and i is index in input.
    pub(crate) fn set_paren_start(&self, group_nr: usize, position: usize) {
        self.state
            .borrow_mut()
            .capture_state
            .set_paren_start(group_nr, position)
    }

    pub(crate) fn get_paren_end(&self, group_nr: usize) -> Option<usize> {
        if group_nr < self.state.borrow().capture_state.endn.len() {
            return self.state.borrow().capture_state.endn[group_nr];
        }
        None
    }

    pub(crate) fn set_paren_end(&self, group_nr: usize, position: usize) {
        self.state
            .borrow_mut()
            .capture_state
            .set_paren_end(group_nr, position)
    }

    pub(crate) fn clear_captured_groups_beyond(&self, pos: usize) {
        for i in 0..self.startn_len() {
            let start = self.capture_state_startn(i);
            if start >= Some(pos) {
                self.set_capture_state_endn(i, start);
            }
        }
        for i in 0..self.start_backref_len() {
            let start = self.start_backref(i);
            if start >= Some(pos) {
                self.set_end_backref(i, start);
            }
        }
    }

    fn capture_state_startn(&self, i: usize) -> Option<usize> {
        self.state.borrow().capture_state.startn[i]
    }

    fn set_capture_state_endn(&self, i: usize, value: Option<usize>) {
        self.state.borrow_mut().capture_state.endn[i] = value;
    }

    pub(crate) fn paren_count(&self) -> usize {
        self.state.borrow().capture_state.paren_count
    }

    pub(crate) fn set_paren_count(&self, count: usize) {
        self.state.borrow_mut().capture_state.paren_count = count;
    }

    pub(crate) fn capture_state(&self) -> CaptureState {
        self.state.borrow().capture_state.clone()
    }

    pub(crate) fn reset_state(&self, capture_state: CaptureState) {
        self.state.borrow_mut().capture_state = capture_state;
    }
}

#[derive(Debug, Clone)]
pub(crate) struct CaptureState {
    // Number of subexpressions matched (num open parens + 1)
    pub(crate) paren_count: usize,
    // Lazily-allocated array of sub-expression starts
    pub(crate) startn: Vec<Option<usize>>,
    // Lazily-allocated array of sub-expression ends
    pub(crate) endn: Vec<Option<usize>>,
}

impl CaptureState {
    fn new() -> Self {
        Self {
            paren_count: 0,
            startn: vec![None, None, None],
            endn: vec![None, None, None],
        }
    }

    pub(crate) fn set_paren_start(&mut self, group_nr: usize, position: usize) {
        // if we use a group nr that' hns bigger than the len
        // In the Java version this has complicated array doubling and copying
        // code, but it appears it can be just extended with None values.
        while group_nr >= self.startn.len() {
            self.startn.extend(vec![None; self.startn.len()]);
        }
        self.startn[group_nr] = Some(position);
    }

    pub(crate) fn set_paren_end(&mut self, group_nr: usize, position: usize) {
        // see set_paren_start
        while group_nr > self.endn.len() - 1 {
            self.endn.extend(vec![None; self.endn.len()]);
        }
        self.endn[group_nr] = Some(position);
    }
}
