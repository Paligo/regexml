use crate::{
    operation::{OperationControl, MATCHES_ZLS_ANYWHERE},
    re_matcher::ReMatcher,
};

#[derive(Debug)]
pub(crate) struct Nothing;

impl OperationControl for Nothing {
    fn get_match_length(&self) -> Option<usize> {
        Some(0)
    }

    fn matches_empty_string(&self) -> u32 {
        MATCHES_ZLS_ANYWHERE
    }

    fn matches_iter(
        &self,
        _matcher: &ReMatcher,
        position: usize,
    ) -> Box<dyn Iterator<Item = usize>> {
        Box::new(std::iter::once(position))
    }
}
