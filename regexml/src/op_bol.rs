use std::rc::Rc;

use crate::operation::{Operation, OperationControl, MATCHES_ZLS_AT_START};
use crate::re_flags::ReFlags;
use crate::re_matcher::ReMatcher;

// Beginning of Line (^) in a regular expression
#[derive(Debug, Clone)]
pub(crate) struct Bol;

impl OperationControl for Bol {
    fn get_match_length(&self) -> Option<usize> {
        Some(0)
    }

    fn matches_empty_string(&self) -> u32 {
        MATCHES_ZLS_AT_START
    }

    fn optimize(&self, _flags: &ReFlags) -> Rc<Operation> {
        Rc::new(Operation::from(self.clone()))
    }

    fn matches_iter<'a>(
        &self,
        matcher: &'a ReMatcher,
        position: usize,
    ) -> Box<dyn Iterator<Item = usize> + 'a> {
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
}
