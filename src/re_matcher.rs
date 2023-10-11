use std::cell::RefCell;

use ahash::{HashMap, HashMapExt, HashSet};

use crate::{
    operation::{Operation, OperationControl},
    re_program::{ReProgram, OPT_HASBACKREFS, OPT_HASBOL},
};

pub(crate) struct ReMatcher<'a> {
    // current program
    pub(crate) program: ReProgram,
    // string being matched against
    pub(crate) search: &'a [char],
    pub(crate) history: History,
    pub(crate) max_paren: Option<usize>,
    // parenthesized subexpressions
    pub(crate) state: RefCell<State>,
}

pub(crate) struct History {
    zero_length_matches: HashMap<Operation, HashSet<char>>,
}

impl History {
    pub(crate) fn new() -> Self {
        Self {
            zero_length_matches: HashMap::new(),
        }
    }

    pub(crate) fn is_duplicate_zero_length_match(
        &self,
        operation: &Operation,
        position: usize,
    ) -> bool {
        todo!()
    }
}

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
    pub(crate) fn new(program: ReProgram) -> Self {
        let max_paren = program.max_parens;
        Self {
            program,
            search: &[],
            history: History::new(),
            max_paren,
            state: RefCell::new(State::new()),
        }
    }

    pub(crate) fn get_paren_count(&self) -> usize {
        self.state.borrow().capture_state.paren_count
    }

    pub(crate) fn get_paren(&self, which: usize) -> Option<&[char]> {
        if which < self.state.borrow().capture_state.paren_count {
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
            s2[..self.start_len()].copy_from_slice(&self.state.borrow().capture_state.startn[..]);
            for entry in s2.iter_mut().skip(self.start_len()) {
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
            s2[..self.end_len()].copy_from_slice(&self.state.borrow().capture_state.endn[..]);
            for entry in s2.iter_mut().skip(self.end_len()) {
                *entry = None
            }
            self.state.borrow_mut().capture_state.endn = s2;
        }
        self.state.borrow_mut().capture_state.startn[which] = Some(i);
    }

    fn end_len(&self) -> usize {
        self.state.borrow().capture_state.endn.len()
    }
    pub(crate) fn clear_captured_groups_beyond(&self, pos: usize) {
        for i in 0..self.start_len() {
            if self.state.borrow().capture_state.startn[i] >= Some(pos) {
                self.state.borrow_mut().capture_state.endn[i] =
                    self.state.borrow().capture_state.startn[i];
            }
            for i in 0..self.state.borrow().start_backref.len() {
                if self.state.borrow().start_backref[i] >= Some(pos) {
                    self.state.borrow_mut().end_backref[i] = self.state.borrow().start_backref[i];
                }
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
                if let Some(nl) = nl {
                    let nl = nl + 1;
                    if nl >= self.search.len() {
                        return false;
                    }
                    if self.match_at(nl, false) {
                        return true;
                    }
                } else {
                    return false;
                }
            }
        }

        // is the string long enough to match?
        let actual_length = self.search.len() - 1;
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

    fn check_preconditions(&self, i: usize) -> bool {
        todo!()
    }

    pub(crate) fn is_new_line(&self, i: usize) -> bool {
        todo!();
    }

    pub(crate) fn equal_case_blind(&self, a: char, b: char) -> bool {
        todo!()
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
