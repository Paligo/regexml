use std::rc::Rc;

use enum_dispatch::enum_dispatch;

use crate::op_atom::Atom;
use crate::op_back_reference::BackReference;
use crate::op_bol::Bol;
use crate::op_capture::Capture;
use crate::op_character_class::CharClass;
use crate::op_choice::Choice;
use crate::op_end_program::EndProgram;
use crate::op_eol::Eol;
use crate::op_greedy_fixed::GreedyFixed;
use crate::op_nothing::Nothing;
use crate::op_reluctant_fixed::ReluctantFixed;
use crate::op_repeat::Repeat;
use crate::op_sequence::Sequence;
use crate::op_unambiguous_repeat::UnambiguousRepeat;

use crate::re_matcher::ReMatcher;

pub(crate) const MATCHES_ZLS_AT_START: u32 = 1;
pub(crate) const MATCHES_ZLS_AT_END: u32 = 2;
pub(crate) const MATCHES_ZLS_ANYWHERE: u32 = 7;
pub(crate) const MATCHES_ZLS_NEVER: u32 = 1024;

#[enum_dispatch]
pub(crate) trait OperationControl {
    /// Get the length of the matches returned by this operation if they are
    /// fixed-length.
    fn get_match_length(&self) -> Option<usize> {
        None
    }

    /// Get the minimum length of the matches returned by this operation.
    fn get_minimum_match_length(&self) -> usize {
        self.get_match_length().unwrap_or(0)
    }

    /// Ask whether the regular expression is known, after static analysis, to
    /// match a zero-length string.
    ///
    /// Specifically:
    ///
    /// * MATCHES_ZLS_AT_START if the expression is statically known to match a
    ///   zero-length string at the start of the supplied input
    ///
    /// * MATCHES_ZLS_AT_END} if it is statically known to return a zero-length
    ///   string at the end of the supplied input.
    ///
    /// * MATCHES_ZLS_ANYWHERE if it is statically known to match a zero-length
    ///   string anywhere in the input.
    ///
    /// * MATCHES_ZLS_NEVER if it is statically known that the regex will never
    ///   match a zero length string.
    ///
    /// Returning 0 means that it is not known statically whether or not the
    /// regex will match a zero-length string; this case typically arises when
    /// back-references are involved.
    fn matches_empty_string(&self) -> u32;

    /// Get an iterator returning all the matches for this operation.
    ///
    /// The `matcher` supplies the context for the matching; may be updated
    /// with information about captured groups.
    ///
    /// The position is the start position to seek a match.
    fn matches_iter<'a>(
        &self,
        matcher: &'a ReMatcher<'a>,
        position: usize,
    ) -> Box<dyn Iterator<Item = usize> + 'a>;

    // fn optimize(&mut self, program: &ReProgram, flags: &ReFlags) {}

    /// Ask whether the expression contains any capturing sub-expressions
    /// Returns true if the expression contains any capturing sub-expressions
    /// (but not if it is a capturing expression itself, unless it contains
    /// nested capturing expressions).
    fn contains_capturing_expressions(&self) -> bool {
        false
    }

    fn display(&self) -> String;
}

pub(crate) trait RepeatOperation {
    fn child(&self) -> Rc<Operation>;
    fn min(&self) -> usize;
}

// TODO: how come I can derive these here without deriving them for the operations?
// is that correct?
#[enum_dispatch(OperationControl)]
#[derive(Debug)]
pub(crate) enum Operation {
    Bol,
    Atom,
    BackReference,
    Capture,
    Choice,
    EndProgram,
    Eol,
    Nothing,
    Repeat,
    Sequence,
    CharClass,
    GreedyFixed,
    ReluctantFixed,
    UnambiguousRepeat,
}

// blanket implementation for references
// impl<T: Operation> Operation for &T {
//     fn get_match_length(&self) -> Option<usize> {
//         (*self).get_match_length()
//     }

//     fn get_minimum_match_length(&self) -> usize {
//         (*self).get_minimum_match_length()
//     }

//     fn matches_empty_string(&self) -> u32 {
//         (*self).matches_empty_string()
//     }

//     fn matches_iter<'a>(
//         &self,
//         matcher: &'a ReMatcher<'a>,
//         position: usize,
//     ) -> Box<dyn Iterator<Item = usize> + 'a> {
//         (*self).matches_iter(matcher, position)
//     }

//     // fn optimize(&mut self, program: &ReProgram, flags: &ReFlags) {
//     //     (*self).optimize(program, flags)
//     // }

//     fn display(&self) -> String {
//         (*self).display()
//     }
// }

// The ForceProgressIterator is used to protect against non-termination;
// specifically, iterators that return an infinite number of zero-length
// matches. After getting a certain number of zero-length matches at the same
// position, hasNext() returns false. (Potentially this gives problems with an
// expression such as (a?|b?|c?|d) that can legitimately return more than one
// zero-length match).
pub(crate) struct ForceProgressIterator<'a> {
    base: Box<dyn Iterator<Item = usize> + 'a>,
    count_zero_length: usize,
    current_pos: Option<usize>,
}

impl<'a> ForceProgressIterator<'a> {
    pub(crate) fn new(base: Box<dyn Iterator<Item = usize> + 'a>) -> Self {
        Self {
            base,
            count_zero_length: 0,
            current_pos: None,
        }
    }
}
impl<'a> Iterator for ForceProgressIterator<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count_zero_length > 3 {
            return None;
        }
        let p = self.base.next()?;
        if Some(p) == self.current_pos {
            self.count_zero_length += 1;
        } else {
            self.count_zero_length = 0;
            self.current_pos = Some(p);
        }
        Some(p)
    }
}
