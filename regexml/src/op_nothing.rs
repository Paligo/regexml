use std::rc::Rc;

use crate::{
    operation::{Operation, OperationControl, MATCHES_ZLS_ANYWHERE},
    re_flags::ReFlags,
    re_matcher::ReMatcher,
};

// Match empty string within a regular expression
#[derive(Debug, Clone)]
pub(crate) struct Nothing;

impl OperationControl for Nothing {
    fn get_match_length(&self) -> Option<usize> {
        Some(0)
    }

    fn matches_empty_string(&self) -> u32 {
        MATCHES_ZLS_ANYWHERE
    }

    fn optimize(&self, _flags: &ReFlags) -> Rc<Operation> {
        Rc::new(Operation::from(self.clone()))
    }

    fn matches_iter(
        &self,
        _matcher: &ReMatcher,
        position: usize,
    ) -> Box<dyn Iterator<Item = usize>> {
        Box::new(std::iter::once(position))
    }
}
