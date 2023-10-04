use crate::{
    operation::{MatchesZls, Operation},
    re_matcher::ReMatcher,
};

struct OpAtom {
    atom: String,
    len: usize,
}

impl OpAtom {
    pub(crate) fn new(atom: String) -> Self {
        Self {
            len: atom.len(),
            atom,
        }
    }
}

impl Operation for OpAtom {
    fn get_match_length(&self) -> Option<usize> {
        Some(self.len)
    }

    fn matches_empty_string(&self) -> Option<MatchesZls> {
        if self.len == 0 {
            Some(MatchesZls::Anywhere)
        } else {
            Some(MatchesZls::Never)
        }
    }

    // TODO
    // fn get_initial_character_class(case_blind: bool) -> CharacterClass {

    // }

    fn matches_iter(
        &self,
        matcher: &ReMatcher,
        position: usize,
    ) -> Box<dyn Iterator<Item = usize> + '_> {
        let in_ = matcher.search;
        if (position + self.len) > in_.len() {
            return Box::new(std::iter::empty());
        }
        let mut in_chars = in_.iter().skip(position);
        let atom_chars = self.atom.chars();
        if matcher.program.flags.is_case_independent() {
            for atom_char in atom_chars {
                let in_char = in_chars.next().unwrap();
                if !matcher.equal_case_blind(*in_char, atom_char) {
                    return Box::new(std::iter::empty());
                }
            }
        } else {
            for atom_char in atom_chars {
                let in_char = in_chars.next().unwrap();
                if *in_char != atom_char {
                    return Box::new(std::iter::empty());
                }
            }
        }
        Box::new(std::iter::once(position + self.len))
    }

    fn display(&self) -> String {
        self.atom.clone()
    }
}
