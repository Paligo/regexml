use crate::{
    operation::{OperationControl, MATCHES_ZLS_AT_END},
    re_matcher::ReMatcher,
};

#[derive(Debug)]
pub(crate) struct Eol;

impl OperationControl for Eol {
    fn get_match_length(&self) -> Option<usize> {
        Some(0)
    }

    fn matches_empty_string(&self) -> u32 {
        MATCHES_ZLS_AT_END
    }

    fn matches_iter(
        &self,
        matcher: &ReMatcher,
        position: usize,
    ) -> Box<dyn Iterator<Item = usize>> {
        let search = matcher.search;

        if matcher.program.flags.is_multi_line() {
            if search.is_empty() || position >= search.len() || matcher.is_new_line(position) {
                Box::new(std::iter::once(position))
            } else {
                Box::new(std::iter::empty())
            }
        } else if search.is_empty() || position >= search.len() {
            Box::new(std::iter::once(position))
        } else {
            Box::new(std::iter::empty())
        }
    }

    fn display(&self) -> String {
        "$".to_string()
    }
}
