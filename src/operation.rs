use enum_dispatch::enum_dispatch;

use crate::op_atom::OpAtom;
use crate::op_back_reference::OpBackReference;
use crate::op_bol::OpBol;
use crate::op_capture::OpCapture;
use crate::op_character_class::OpCharacterClass;
use crate::op_choice::OpChoice;
use crate::op_end_program::OpEndProgram;
use crate::op_eol::OpEol;
use crate::op_nothing::OpNothing;
use crate::op_repeat::OpRepeat;
use crate::op_sequence::OpSequence;

use crate::{
    re_matcher::ReMatcher,
    re_program::{ReFlags, ReProgram},
};

pub(crate) const MATCHES_ZLS_AT_START: u32 = 1;
pub(crate) const MATCHES_ZLS_AT_END: u32 = 2;
pub(crate) const MATCHES_ZLS_ANYWHERE: u32 = 7;
pub(crate) const MATCHES_ZLS_NEVER: u32 = 1024;

#[enum_dispatch]
pub(crate) trait OperationControl {
    fn get_match_length(&self) -> Option<usize> {
        None
    }

    fn get_minimum_match_length(&self) -> usize {
        self.get_match_length().unwrap_or(0)
    }

    fn matches_empty_string(&self) -> u32;

    fn matches_iter<'a>(
        &self,
        matcher: &'a ReMatcher<'a>,
        position: usize,
    ) -> Box<dyn Iterator<Item = usize> + 'a>;

    // fn optimize(&mut self, program: &ReProgram, flags: &ReFlags) {}

    fn display(&self) -> String;
}

#[enum_dispatch(OperationControl)]
pub(crate) enum Operation {
    OpBol,
    OpAtom,
    OpBackReference,
    OpCapture,
    OpChoice,
    OpEndProgram,
    OpEol,
    OpNothing,
    OpRepeat,
    OpSequence,
    OpCharacterClass,
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
