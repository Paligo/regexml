use crate::{
    operation::{Operation, OperationControl},
    re_flags::ReFlags,
    re_matcher::ReMatcher,
    re_program::OPT_HASBACKREFS,
};

// Open paren (captured group) within a regular expression
#[derive(Debug, Clone)]
pub(crate) struct Capture {
    group_nr: usize,
    pub(crate) child_op: Box<Operation>,
}

impl Capture {
    pub(crate) fn new(group_nr: usize, child_op: Operation) -> Self {
        Self {
            group_nr,
            child_op: Box::new(child_op),
        }
    }
}

impl OperationControl for Capture {
    fn get_match_length(&self) -> Option<usize> {
        self.child_op.get_match_length()
    }

    fn get_minimum_match_length(&self) -> usize {
        self.child_op.get_minimum_match_length()
    }

    fn matches_empty_string(&self) -> u32 {
        self.child_op.matches_empty_string()
    }

    fn optimize(self, flags: &ReFlags) -> Operation {
        Operation::from(Capture {
            group_nr: self.group_nr,
            child_op: Box::new(self.child_op.optimize(flags)),
        })
    }

    fn matches_iter<'a>(
        &'a self,
        matcher: &'a ReMatcher<'a>,
        position: usize,
    ) -> Box<dyn Iterator<Item = usize> + 'a> {
        // everything this attempt captures, here and inside the group, is
        // logged after this mark
        let mark = matcher.trail_mark();
        if (matcher.program.optimization_flags & OPT_HASBACKREFS) != 0 {
            matcher.open_backref(self.group_nr, position);
        }
        let basis = self.child_op.matches_iter(matcher, position);

        Box::new(CaptureGroupIterator::new(
            matcher,
            basis,
            self.group_nr,
            position,
            mark,
        ))
    }

    fn children(&self) -> Vec<Operation> {
        vec![self.child_op.as_ref().clone()]
    }

    fn collect_groups(&self, groups: &mut Vec<usize>) {
        groups.push(self.group_nr);
        self.child_op.collect_groups(groups);
    }
}

struct CaptureGroupIterator<'a> {
    matcher: &'a ReMatcher<'a>,
    basis: Box<dyn Iterator<Item = usize> + 'a>,
    group_nr: usize,
    position: usize,
    // the capture log's length when this attempt began
    mark: usize,
}

impl<'a> CaptureGroupIterator<'a> {
    fn new(
        matcher: &'a ReMatcher<'a>,
        basis: Box<dyn Iterator<Item = usize> + 'a>,
        group_nr: usize,
        position: usize,
        mark: usize,
    ) -> Self {
        Self {
            matcher,
            basis,
            group_nr,
            position,
            mark,
        }
    }
}

impl Iterator for CaptureGroupIterator<'_> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let Some(next) = self.basis.next() else {
            // The group cannot match here (any more): put back what it and
            // the groups inside it held before this attempt, so that a group
            // keeps the last substring it matched on the path that is still
            // being followed, or stays unmatched.
            self.matcher.undo_to(self.mark);
            return None;
        };

        // Increase valid paren count
        if self.group_nr >= self.matcher.paren_count() {
            self.matcher.set_paren_count(self.group_nr + 1);
        }

        self.matcher.capture(self.group_nr, self.position, next);
        Some(next)
    }
}
