use crate::{
    character_class::CharacterClass,
    operation::{Operation, OperationControl, MATCHES_ZLS_NEVER},
    re_flags::ReFlags,
};

// A match of a single character in the input against a set of permitted
// characters
#[derive(Debug, Clone)]
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

    fn get_initial_character_class(&self, _case_blind: bool) -> CharacterClass {
        // TODO: is this correct? can we just ignore case blind?
        self.character_class.clone()
    }

    fn optimize(self, _flags: &ReFlags) -> Operation {
        Operation::from(self)
    }

    fn matches_iter<'b>(
        &self,
        matcher: &'b crate::re_matcher::ReMatcher,
        position: usize,
    ) -> Box<dyn Iterator<Item = usize> + 'b> {
        let search = &matcher.search;
        if position < search.len() && self.character_class.contains(search[position]) {
            Box::new(std::iter::once(position + 1))
        } else {
            Box::new(std::iter::empty())
        }
    }
}
