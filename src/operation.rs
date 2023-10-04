use crate::{
    re_matcher::ReMatcher,
    re_program::{ReFlags, ReProgram},
};

pub(crate) const MATCHES_ZLS_AT_START: u32 = 1;
pub(crate) const MATCHES_ZLS_AT_END: u32 = 2;
pub(crate) const MATCHES_ZLS_ANYWHERE: u32 = 7;
pub(crate) const MATCHES_ZLS_NEVER: u32 = 1024;

pub(crate) trait Operation {
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

// blanket implementation for references
impl<T: Operation> Operation for &T {
    fn get_match_length(&self) -> Option<usize> {
        (*self).get_match_length()
    }

    fn get_minimum_match_length(&self) -> usize {
        (*self).get_minimum_match_length()
    }

    fn matches_empty_string(&self) -> u32 {
        (*self).matches_empty_string()
    }

    fn matches_iter<'a>(
        &self,
        matcher: &'a ReMatcher<'a>,
        position: usize,
    ) -> Box<dyn Iterator<Item = usize> + 'a> {
        (*self).matches_iter(matcher, position)
    }

    // fn optimize(&mut self, program: &ReProgram, flags: &ReFlags) {
    //     (*self).optimize(program, flags)
    // }

    fn display(&self) -> String {
        (*self).display()
    }
}
