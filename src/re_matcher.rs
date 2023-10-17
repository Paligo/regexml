use std::cell::RefCell;

use ahash::{HashMap, HashMapExt, HashSet, HashSetExt};

use crate::{
    operation::{Operation, OperationControl},
    re_compiler::Error,
    re_program::{ReProgram, OPT_HASBACKREFS, OPT_HASBOL},
};

pub(crate) struct ReMatcher<'a> {
    // current program
    pub(crate) program: &'a ReProgram,
    // string being matched against
    pub(crate) search: &'a [char],
    pub(crate) max_paren: Option<usize>,
    // parenthesized subexpressions
    pub(crate) state: RefCell<State>,
}

pub(crate) struct History {
    zero_length_matches: HashMap<Operation, HashSet<usize>>,
}

impl History {
    pub(crate) fn new() -> Self {
        Self {
            zero_length_matches: HashMap::new(),
        }
    }

    pub(crate) fn is_duplicate_zero_length_match(
        &mut self,
        operation: Operation,
        position: usize,
    ) -> bool {
        // TODO: make this hashable
        false

        // TODO: hashing an operation; how can that work with enum dispatch?
        // let positions = self.zero_length_matches.get_mut(&operation);
        // if let Some(positions) = positions {
        //     if positions.contains(&position) {
        //         true
        //     } else {
        //         positions.insert(position);
        //         false
        //     }
        // } else {
        //     let mut positions = HashSet::new();
        //     positions.insert(position);
        //     self.zero_length_matches.insert(operation, positions);
        //     false
        // }
    }
}

pub(crate) struct State {
    pub(crate) start_backref: Vec<Option<usize>>,
    pub(crate) end_backref: Vec<Option<usize>>,
    pub(crate) capture_state: CaptureState,
    pub(crate) anchored_match: bool,
    pub(crate) history: History,
}

impl State {
    pub(crate) fn new() -> Self {
        Self {
            start_backref: vec![],
            end_backref: vec![],
            capture_state: CaptureState::new(),
            anchored_match: false,
            history: History::new(),
        }
    }
}

impl<'a> ReMatcher<'a> {
    pub(crate) fn new(program: &'a ReProgram) -> Self {
        let max_paren = program.max_parens;
        Self {
            program,
            search: &[],
            max_paren,
            state: RefCell::new(State::new()),
        }
    }

    pub(crate) fn get_paren_count(&self) -> usize {
        self.state.borrow().capture_state.paren_count
    }

    pub(crate) fn get_paren(&self, which: usize) -> Option<&[char]> {
        if which < self.get_paren_count() {
            if let (Some(start), Some(end)) =
                (self.get_paren_start(which), self.get_paren_end(which))
            {
                return Some(&self.search[start..end]);
            }
        }
        None
    }

    pub(crate) fn get_paren_start(&self, which: usize) -> Option<usize> {
        if which < self.state.borrow().capture_state.startn.len() {
            return self.state.borrow().capture_state.startn[which];
        }
        None
    }

    pub(crate) fn get_paren_end(&self, which: usize) -> Option<usize> {
        if which < self.state.borrow().capture_state.endn.len() {
            return self.state.borrow().capture_state.endn[which];
        }
        None
    }

    pub(crate) fn set_paren_start(&self, which: usize, i: usize) {
        while which > self.start_len() - 1 {
            let mut s2 = Vec::with_capacity(self.start_len() * 2);
            let start_len = self.start_len();
            s2[..start_len].copy_from_slice(&self.state.borrow().capture_state.startn[..start_len]);
            for entry in s2.iter_mut().skip(start_len) {
                *entry = None
            }
            self.state.borrow_mut().capture_state.startn = s2;
        }
        self.state.borrow_mut().capture_state.startn[which] = Some(i);
    }

    fn start_len(&self) -> usize {
        self.state.borrow().capture_state.startn.len()
    }

    pub(crate) fn set_paren_end(&self, which: usize, i: usize) {
        while which > self.end_len() - 1 {
            let mut s2 = Vec::with_capacity(self.end_len() * 2);
            let end_len = self.end_len();
            s2[..end_len].copy_from_slice(&self.state.borrow().capture_state.endn[..end_len]);
            for entry in s2.iter_mut().skip(end_len) {
                *entry = None
            }
            self.state.borrow_mut().capture_state.endn = s2;
        }
        self.state.borrow_mut().capture_state.endn[which] = Some(i);
    }

    fn end_len(&self) -> usize {
        self.state.borrow().capture_state.endn.len()
    }

    pub(crate) fn reset_state(&self, capture_state: CaptureState) {
        self.state.borrow_mut().capture_state = capture_state;
    }

    pub(crate) fn clear_captured_groups_beyond(&self, pos: usize) {
        for i in 0..self.start_len() {
            let start = self.state.borrow().capture_state.startn[i];
            if start >= Some(pos) {
                self.state.borrow_mut().capture_state.endn[i] = start;
            }
        }
        for i in 0..self.state.borrow().start_backref.len() {
            let start = self.state.borrow().start_backref[i];
            if start >= Some(pos) {
                self.state.borrow_mut().end_backref[i] = start;
            }
        }
    }

    pub(crate) fn match_at(&self, i: usize, anchored: bool) -> bool {
        // initialize start pointer, paren cache and paren count
        self.state.borrow_mut().capture_state.paren_count = 1;
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

    // an 'is' function that mutates self? weird
    pub(crate) fn is_anchored_match(&mut self, search: &'a [char]) -> bool {
        self.search = search;
        self.match_at(0, true)
    }

    pub(crate) fn matches(&mut self, search: &'a [char], i: usize) -> bool {
        // save string to search
        self.search = search;

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
            let mut nl = Some(i);
            loop {
                // search for position from previous nl
                nl = if let Some(skip) = nl {
                    self.search.iter().skip(skip).position(|c| *c == '\n')
                } else {
                    None
                };
                if let Some(n) = nl {
                    let n = n + 1;
                    nl = Some(n);
                    if n >= self.search.len() {
                        return false;
                    } else if self.match_at(n, false) {
                        return true;
                    }
                } else {
                    return false;
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
                        if search[k + j] != prefix[k] {
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
            if let Some(pred) = self.program.initial_character_class() {
                for j in i..self.search.len() {
                    if pred.test(self.search[j]) && self.match_at(j, false) {
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
            for j in i..(search.len() + 1) {
                // try a match at index i
                if self.match_at(j, false) {
                    return true;
                }
            }
            false
        }
    }

    pub(crate) fn is_match(&mut self, search: &'a [char]) -> bool {
        self.matches(search, 0)
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

    pub(crate) fn replace(
        &mut self,
        input: &'a [char],
        replacement: &[char],
    ) -> Result<Vec<char>, Error> {
        // string to return
        let mut result = Vec::new();

        // start at position 0 and search the whole string
        let mut pos = 0;
        let len = input.len();

        let mut first_match = true;
        let mut simple_replacement = false;

        // try a match at each position
        while pos < len && self.matches(input, pos) {
            // append chars from input string before match
            if let Some(start) = self.get_paren_start(0) {
                result.extend(&input[pos..start]);
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
                            let ch = replacement[index];
                            match ch {
                                '\\' | '$' => {
                                    result.push(ch);
                                }
                                _ => {
                                    return Err(Error::syntax(format!(
                                        "Invalid escape '{}' in replacement string",
                                        ch
                                    )))
                                }
                            }
                        }
                        '$' => {
                            simple_replacement = false;
                            i += 1;
                            let index = i;
                            let ch = replacement[index];
                            if !ch.is_ascii_digit() {
                                return Err(Error::syntax(
                                    "$ in replacement string must be followed by a digit",
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
                                    if i > replacement.len() {
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
            return Ok(input.to_vec());
        }

        // if there's remaining input, append it
        result.extend(&input[pos..len]);

        // return the string buffer
        Ok(result)
    }

    pub(crate) fn is_new_line(&self, i: usize) -> bool {
        self.search[i] == '\n'
    }

    pub(crate) fn equal_case_blind(&self, a: char, b: char) -> bool {
        if a == b {
            return true;
        }
        let uppercase = a.to_uppercase().collect::<Vec<_>>();
        if uppercase.len() == 1 && uppercase[0] == b {
            return true;
        }
        let lowercase = a.to_lowercase().collect::<Vec<_>>();
        if lowercase.len() == 1 {
            return lowercase[0] == b;
        }
        false
    }
}

#[derive(Debug, Clone)]
pub(crate) struct CaptureState {
    // Number of subexpressions matched (num open parens + 1)
    pub(crate) paren_count: usize,
    // TODO can they be arrays instead of vecs?
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
}
