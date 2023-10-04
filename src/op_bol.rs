use crate::operation::Operation;
use crate::re_matcher::ReMatcher;

struct OpBol {}

const MATCHES_ZLS_AT_START: usize = 1;

impl Operation for OpBol {
    fn get_match_length() -> usize {
        0
    }

    fn matches_empty_string() -> usize {
        MATCHES_ZLS_AT_START
    }

    fn matches_iter(
        &self,
        matcher: &mut ReMatcher,
        position: usize,
    ) -> Box<dyn Iterator<Item = usize> + '_> {
        // Fail if we're not at the start of the string
        if position != 0 {
            // If we're multiline matching, we could still be at the start of a line
            if matcher.program.flags.is_multi_line() {
                // Continue if at the start of a line
                if matcher.is_new_line(position - 1) && position < matcher.search.len() {
                    return Box::new(vec![position].into_iter());
                }
            }
            return Box::new(vec![].into_iter());
        }
        Box::new(vec![position].into_iter())
    }

    fn display() -> String {
        "^".to_string()
    }
}
