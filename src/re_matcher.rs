use std::cell::RefCell;

use ahash::{HashMap, HashMapExt, HashSet};

use crate::{
    operation::{Operation, OperationControl},
    re_program::ReProgram,
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

    // back references
    start_backref: Vec<Option<usize>>,
    end_backref: Vec<Option<usize>>,

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
    pub(crate) start_back_ref: Vec<Option<usize>>,
    pub(crate) end_back_ref: Vec<Option<usize>>,
    pub(crate) capture_state: CaptureState,
}

impl State {
    pub(crate) fn new() -> Self {
        Self {
            start_back_ref: vec![],
            end_back_ref: vec![],
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
            start_backref: vec![],
            end_backref: vec![],
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
        todo!()
    }
    pub(crate) fn set_paren_end(&self, which: usize, i: usize) {
        todo!()
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

    pub(crate) fn clear_captured_groups_beyond(&self, position: usize) {}
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
