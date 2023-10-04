use crate::{
    re_matcher::ReMatcher,
    re_program::{ReFlags, ReProgram},
};

pub(crate) enum MatchesZls {
    AtStart,
    AtEnd,
    Anywhere,
    Never,
}

pub(crate) trait Operation {
    fn get_match_length(&self) -> Option<usize> {
        None
    }

    fn get_minimum_match_length(&self) -> usize {
        self.get_match_length().unwrap_or(0)
    }

    fn matches_empty_string(&self) -> Option<MatchesZls>;

    fn matches_iter<'a>(
        &self,
        matcher: &'a ReMatcher<'a>,
        position: usize,
    ) -> Box<dyn Iterator<Item = usize> + 'a>;

    // fn optimize(&mut self, program: &ReProgram, flags: &ReFlags) {}

    fn display(&self) -> String;
}
