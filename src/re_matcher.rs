use std::cell::RefCell;

use ahash::{HashMap, HashMapExt, HashSet};

use crate::{
    operation::{Operation, OperationControl},
    re_program::{ReProgram, OPT_HASBACKREFS},
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

    pub(crate) anchored_match: bool,
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
}

impl State {
    pub(crate) fn new() -> Self {
        Self {
            start_backref: vec![],
            end_backref: vec![],
            capture_state: CaptureState::new(),
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
            anchored_match: false,
        }
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

    pub(crate) fn match_at(&mut self, i: usize, anchored: bool) -> bool {
        // initialize start pointer, paren cache and paren count
        self.state.borrow_mut().capture_state.paren_count = 1;
        self.anchored_match = anchored;
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

    pub(crate) fn get_paren_count(&self) -> usize {
        todo!()
    }

    pub(crate) fn matches(&self, str: &[char], start: usize) -> bool {
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
