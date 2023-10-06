use crate::{
    operation::{OperationControl, MATCHES_ZLS_ANYWHERE},
    re_matcher::ReMatcher,
};

pub(crate) struct OpNothing;

impl OperationControl for OpNothing {
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

    fn display(&self) -> String {
        "()".to_string()
    }
}
