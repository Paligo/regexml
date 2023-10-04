use crate::re_matcher::ReMatcher;

pub(crate) trait Operation {
    fn get_match_length() -> usize;
    fn matches_empty_string() -> usize;

    fn matches_iter(
        &self,
        matcher: &mut ReMatcher,
        position: usize,
    ) -> Box<dyn Iterator<Item = usize> + '_>;

    fn display() -> String;
}
