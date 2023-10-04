use crate::{
    operation::Operation,
    re_matcher::ReMatcher,
    re_program::{ReFlags, ReProgram, OPT_HASBACKREFS},
};

struct OpCapture {
    group_nr: usize,
    child_op: Box<dyn Operation>,
}

impl OpCapture {
    pub(crate) fn new(group_nr: usize, child_op: Box<dyn Operation>) -> Self {
        Self { group_nr, child_op }
    }
}

impl Operation for OpCapture {
    fn get_match_length(&self) -> Option<usize> {
        self.child_op.get_match_length()
    }

    fn get_minimum_match_length(&self) -> usize {
        self.child_op.get_minimum_match_length()
    }

    fn matches_empty_string(&self) -> u32 {
        self.child_op.matches_empty_string()
    }

    // fn optimize(&mut self, program: &ReProgram, flags: &ReFlags) {
    //     self.child_op.optimize(program, flags);
    // }

    fn matches_iter<'a>(
        &self,
        matcher: &'a ReMatcher<'a>,
        position: usize,
    ) -> Box<dyn Iterator<Item = usize> + 'a> {
        if (matcher.program.optimization_flags & OPT_HASBACKREFS) != 0 {
            matcher.state.borrow_mut().start_back_ref[self.group_nr] = Some(position);
        }
        let basis = self.child_op.matches_iter(matcher, position);

        Box::new(CaptureGroupIterator::new(
            matcher,
            basis,
            self.group_nr,
            position,
        ))
    }

    fn display(&self) -> String {
        format!("({})", self.child_op.display())
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
        if self.group_nr >= self.matcher.state.borrow().capture_state.paren_count {
            self.matcher.state.borrow_mut().capture_state.paren_count = self.group_nr + 1;
        }

        // Don't set paren if already set later on
        self.matcher.set_paren_start(self.group_nr, self.position);
        self.matcher.set_paren_end(self.group_nr, next);

        if (self.matcher.program.optimization_flags & OPT_HASBACKREFS) != 0 {
            let mut state = self.matcher.state.borrow_mut();
            state.start_back_ref[self.group_nr] = Some(self.position);
            state.end_back_ref[self.group_nr] = Some(next);
        }
        Some(next)
    }
}
