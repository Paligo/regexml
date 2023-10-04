use crate::operation::{MatchesZls, Operation};
use crate::re_matcher::ReMatcher;

struct OpBol {}

const MATCHES_ZLS_AT_START: usize = 1;

impl Operation for OpBol {
    fn get_match_length(&self) -> Option<usize> {
        Some(0)
    }

    fn matches_empty_string(&self) -> Option<MatchesZls> {
        Some(MatchesZls::AtStart)
    }

    fn matches_iter(
        &self,
        matcher: &ReMatcher,
        position: usize,
    ) -> Box<dyn Iterator<Item = usize> + '_> {
        // Fail if we're not at the start of the string
        if position != 0 {
            // If we're multiline matching, we could still be at the start of a line
            if matcher.program.flags.is_multi_line() {
                // Continue if at the start of a line
                if matcher.is_new_line(position - 1) && position < matcher.search.len() {
                    return Box::new(std::iter::once(position));
                }
            }
            return Box::new(std::iter::empty());
        }
        Box::new(std::iter::once(position))
    }

    fn display(&self) -> String {
        "^".to_string()
    }
}
