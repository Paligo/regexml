use std::cell::RefCell;

use crate::{operation::Operation, re_program::ReProgram};

pub(crate) struct ReMatcher<'a> {
    pub(crate) search: &'a [char],
    pub(crate) program: ReProgram,
    pub(crate) state: RefCell<State>,
    pub(crate) history: History,
    pub(crate) anchored_match: bool,
}

pub(crate) struct History {}

impl History {
    pub(crate) fn is_duplicate_zero_length_match(
        &self,
        operation: Box<dyn Operation + '_>,
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

impl<'a> ReMatcher<'a> {
    pub(crate) fn matches(&self, str: &[char], start: usize) -> bool {
        todo!()
    }

    pub(crate) fn get_paren_start(&self, i: usize) -> usize {
        todo!()
    }

    pub(crate) fn get_paren_end(&self, i: usize) -> usize {
        todo!()
    }
    pub(crate) fn get_paren_count(&self) -> usize {
        todo!()
    }

    pub(crate) fn get_paren(&self, i: usize) -> Option<&'a str> {
        todo!()
    }

    pub(crate) fn is_new_line(&self, i: usize) -> bool {
        todo!();
    }

    pub(crate) fn equal_case_blind(&self, a: char, b: char) -> bool {
        todo!()
    }

    pub(crate) fn set_paren_start(&self, which: usize, i: usize) {
        todo!()
    }
    pub(crate) fn set_paren_end(&self, which: usize, i: usize) {
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
