use enum_dispatch::enum_dispatch;

use crate::character_class::CharacterClass;
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

use crate::re_flags::ReFlags;
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

    /// Get a character class identifying the set of the characters that can
    /// appear as the first character of a non-empty string that matches this
    /// term. This is allowed to be an over-estimate (that is, the returned
    /// Character class must match every character that can legitimately appear
    /// at the start of the matched string, but it can also match other
    /// characters).
    fn get_initial_character_class(&self, _case_blind: bool) -> CharacterClass {
        CharacterClass::all()
    }

    /// Get an optimized version of this operation.
    ///
    // TODO: I don't know how to spell a generic one for this, which just
    // clones the operation, so I've replicated them for the various structs.
    fn optimize(self, _flags: &ReFlags) -> Operation;

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
        &'a self,
        matcher: &'a ReMatcher<'a>,
        position: usize,
    ) -> Box<dyn Iterator<Item = usize> + 'a>;

    /// Ask whether the expression contains any capturing sub-expressions
    /// Returns true if the expression contains any capturing sub-expressions
    /// (but not if it is a capturing expression itself, unless it contains
    /// nested capturing expressions).
    fn contains_capturing_expressions(&self) -> bool {
        false
    }

    /// Append the numbers of the capturing groups in this expression,
    /// including this one if it is a capturing group itself.
    fn collect_groups(&self, _groups: &mut Vec<usize>) {}

    /// Access child information so we can structurally dive into
    /// a regex, mostly for testing purposes.
    /// allow unused code to exist as we do use it in the tests
    #[allow(dead_code)]
    fn children(&self) -> Vec<Operation> {
        Vec::new()
    }
}

pub(crate) trait RepeatOperation {
    fn child(&self) -> Operation;
    fn min(&self) -> usize;
    fn max(&self) -> usize;
    fn greedy(&self) -> bool;
}

#[enum_dispatch(OperationControl)]
#[derive(Debug, Clone)]
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

impl Operation {
    pub(crate) fn repeat_operation(&self) -> Option<&dyn RepeatOperation> {
        match self {
            Operation::Repeat(repeat) => Some(repeat),
            Operation::GreedyFixed(greedy_fixed) => Some(greedy_fixed),
            Operation::ReluctantFixed(reluctant_fixed) => Some(reluctant_fixed),
            Operation::UnambiguousRepeat(unambiguous_repeat) => Some(unambiguous_repeat),
            _ => None,
        }
    }
}

/// Wraps the iterator of a repetition whose repeated operation captures
/// groups: when it gives up, every capture written since it was created is
/// taken back (`ReMatcher::undo_to`).
///
/// A capturing group restores its own captures when it is exhausted
/// (`CaptureGroupIterator`), and a sequence or a choice only moves on from a
/// sub-iterator once it is exhausted, so neither needs this. A repetition
/// does: it keeps the captures of iterations whose iterators it has dropped
/// (they are on the path it reports), and it may stop before those are
/// exhausted (`ForceProgressIterator`), so when it fails, it must undo them
/// itself.
pub(crate) struct RestoreCaptures<'a> {
    matcher: &'a ReMatcher<'a>,
    mark: usize,
    base: Box<dyn Iterator<Item = usize> + 'a>,
}

impl<'a> RestoreCaptures<'a> {
    /// `mark` is `matcher.trail_mark()` taken before the repetition did any
    /// matching (it primes its iterators when it is created).
    pub(crate) fn wrap(
        matcher: &'a ReMatcher<'a>,
        mark: usize,
        base: Box<dyn Iterator<Item = usize> + 'a>,
    ) -> Box<dyn Iterator<Item = usize> + 'a> {
        Box::new(Self {
            matcher,
            mark,
            base,
        })
    }
}

impl Iterator for RestoreCaptures<'_> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.base.next();
        if next.is_none() {
            self.matcher.undo_to(self.mark);
        }
        next
    }
}

/// Whether `operation` is a capturing group or contains one: only then can
/// matching it write captures.
pub(crate) fn captures(operation: &Operation) -> bool {
    matches!(operation, Operation::Capture(_)) || operation.contains_capturing_expressions()
}

/// The numbers of the capturing groups in `operation`.
pub(crate) fn groups_of(operation: &Operation) -> Vec<usize> {
    let mut groups = Vec::new();
    operation.collect_groups(&mut groups);
    groups
}

/// Called when a repetition reports that it has matched its operand zero
/// times: the groups inside the operand did not match in it, so the ones
/// still holding a capture from an earlier iteration of an enclosing
/// repetition are set back to "not matched" (logged, so backtracking past
/// the zero-occurrence choice restores them). Returns the capture log's
/// length before, for `ReMatcher::undo_to`.
///
/// This is how the W3C XQuery/XPath 3 test suite reads captures in nested
/// repetitions (fn-matches-50, `^((.)?a\2)+$` matches "babadad": in the second
/// iteration `(.)?` matches nothing, so `\2` matches the zero-length string;
/// W3C bug 29253); F&O 3.1 §5.6.1 leaves the substring captured in nested
/// repeating constructs implementation-dependent.
pub(crate) fn unbind_groups(matcher: &ReMatcher, groups: &[usize]) -> usize {
    let mark = matcher.trail_mark();
    for &group_nr in groups {
        matcher.unbind(group_nr);
    }
    mark
}

// The ForceProgressIterator is used to protect against non-termination;
// specifically, iterators that return an infinite number of zero-length
// matches. After getting a certain number of zero-length matches at the same
// position, next returns None. (Potentially this gives problems with an
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
impl Iterator for ForceProgressIterator<'_> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count_zero_length > 3 {
            return None;
        }
        let p = Some(self.base.next()?);
        if p == self.current_pos {
            self.count_zero_length += 1;
        } else {
            self.count_zero_length = 0;
            self.current_pos = p;
        }
        p
    }
}
