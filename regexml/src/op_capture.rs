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
        if (matcher.program.optimization_flags & OPT_HASBACKREFS) != 0 {
            matcher.set_start_backref(self.group_nr, Some(position));
        }
        let basis = self.child_op.matches_iter(matcher, position);

        Box::new(CaptureGroupIterator::new(
            matcher,
            basis,
            self.group_nr,
            position,
        ))
    }

    fn children(&self) -> Vec<Operation> {
        vec![self.child_op.as_ref().clone()]
    }
}

struct CaptureGroupIterator<'a> {
    matcher: &'a ReMatcher<'a>,
    basis: Box<dyn Iterator<Item = usize> + 'a>,
    group_nr: usize,
    position: usize,
}

impl<'a> CaptureGroupIterator<'a> {
    fn new(
        matcher: &'a ReMatcher<'a>,
        basis: Box<dyn Iterator<Item = usize> + 'a>,
        group_nr: usize,
        position: usize,
    ) -> Self {
        Self {
            matcher,
            basis,
            group_nr,
            position,
        }
    }
}

impl<'a> Iterator for CaptureGroupIterator<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.basis.next()?;

        // Increase valid paren count
        if self.group_nr >= self.matcher.paren_count() {
            self.matcher.set_paren_count(self.group_nr + 1);
        }

        self.matcher.set_paren_start(self.group_nr, self.position);
        self.matcher.set_paren_end(self.group_nr, next);

        if (self.matcher.program.optimization_flags & OPT_HASBACKREFS) != 0 {
            self.matcher
                .set_start_backref(self.group_nr, Some(self.position));
            self.matcher.set_end_backref(self.group_nr, Some(next));
        }
        Some(next)
    }
}
