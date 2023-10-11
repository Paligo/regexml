use crate::{
    operation::{OperationControl, MATCHES_ZLS_ANYWHERE},
    re_matcher::ReMatcher,
};

pub(crate) struct EndProgram;

impl OperationControl for EndProgram {
    fn get_match_length(&self) -> Option<usize> {
        Some(0)
    }

    fn matches_empty_string(&self) -> u32 {
        MATCHES_ZLS_ANYWHERE
    }

    fn matches_iter(
        &self,
        matcher: &ReMatcher,
        position: usize,
    ) -> Box<dyn Iterator<Item = usize>> {
        // An anchored match is successful only if we are at the end of the
        // string. Otherwise, match has succeeded unconditionally
        if matcher.state.borrow().anchored_match {
            if position >= matcher.search.len() {
                Box::new(std::iter::once(position))
            } else {
                Box::new(std::iter::empty())
            }
        } else {
            matcher.set_paren_end(0, position);
            Box::new(std::iter::once(position))
        }
    }

    fn display(&self) -> String {
        "\\Z".to_string()
    }
}
