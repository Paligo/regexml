use crate::{
    character_class::CharacterClass,
    operation::{OperationControl, MATCHES_ZLS_NEVER},
};

#[derive(Debug)]
pub(crate) struct CharClass {
    pub(crate) character_class: CharacterClass,
}

impl CharClass {
    pub(crate) fn new(character_class: CharacterClass) -> Self {
        Self { character_class }
    }
}

impl OperationControl for CharClass {
    fn get_match_length(&self) -> Option<usize> {
        Some(1)
    }

    fn matches_empty_string(&self) -> u32 {
        MATCHES_ZLS_NEVER
    }

    fn matches_iter<'a>(
        &self,
        matcher: &'a crate::re_matcher::ReMatcher,
        position: usize,
    ) -> Box<dyn Iterator<Item = usize> + 'a> {
        let search = matcher.search;
        if position < search.len() && self.character_class.test(search[position]) {
            Box::new(std::iter::once(position + 1))
        } else {
            Box::new(std::iter::empty())
        }
    }

    fn display(&self) -> String {
        todo!()
    }
}
