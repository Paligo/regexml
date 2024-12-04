use crate::{
    operation::{Operation, OperationControl, MATCHES_ZLS_ANYWHERE},
    re_flags::ReFlags,
    re_matcher::ReMatcher,
};

// End of program in a regular expression
#[derive(Debug, Clone)]
pub(crate) struct EndProgram;

impl OperationControl for EndProgram {
    fn get_match_length(&self) -> Option<usize> {
        Some(0)
    }

    fn matches_empty_string(&self) -> u32 {
        MATCHES_ZLS_ANYWHERE
    }

    fn optimize(self, _flags: &ReFlags) -> Operation {
        Operation::from(self)
    }

    fn matches_iter(
        &self,
        matcher: &ReMatcher,
        position: usize,
    ) -> Box<dyn Iterator<Item = usize>> {
        // An anchored match is successful only if we are at the end of the
        // string. Otherwise, match has succeeded unconditionally
        if matcher.anchored_match() {
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
}
