use crate::re_matcher::ReMatcher;

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
    fn matches_empty_string(&self) -> Option<MatchesZls>;

    fn matches_iter(
        &self,
        matcher: &ReMatcher,
        position: usize,
    ) -> Box<dyn Iterator<Item = usize> + '_>;

    fn display(&self) -> String;
}
