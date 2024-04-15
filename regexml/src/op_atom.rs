use crate::{
    operation::{OperationControl, MATCHES_ZLS_ANYWHERE, MATCHES_ZLS_NEVER},
    re_matcher::ReMatcher,
};

/// A match against a fixed string of any length, within a regular expression.
#[derive(Debug)]
pub(crate) struct Atom {
    pub(crate) atom: Vec<char>,
    len: usize,
}

impl Atom {
    pub(crate) fn new(atom: Vec<char>) -> Self {
        Self {
            len: atom.len(),
            atom,
        }
    }
}

impl OperationControl for Atom {
    fn get_match_length(&self) -> Option<usize> {
        Some(self.len)
    }

    fn matches_empty_string(&self) -> u32 {
        if self.len == 0 {
            MATCHES_ZLS_ANYWHERE
        } else {
            MATCHES_ZLS_NEVER
        }
    }

    // TODO
    // fn get_initial_character_class(case_blind: bool) -> CharacterClass {

    // }

    fn matches_iter<'a>(
        &self,
        matcher: &'a ReMatcher,
        position: usize,
    ) -> Box<dyn Iterator<Item = usize> + 'a> {
        let in_ = &matcher.search;
        if (position + self.len) > in_.len() {
            return Box::new(std::iter::empty());
        }
        let mut in_chars = in_.iter().skip(position);
        let atom_chars = &self.atom;
        if matcher.program.flags.is_case_independent() {
            for atom_char in atom_chars {
                let in_char = in_chars.next().unwrap();
                if !matcher.equal_case_blind(*in_char, *atom_char) {
                    return Box::new(std::iter::empty());
                }
            }
        } else {
            for atom_char in atom_chars {
                let in_char = in_chars.next().unwrap();
                if *in_char != *atom_char {
                    return Box::new(std::iter::empty());
                }
            }
        }
        Box::new(std::iter::once(position + self.len))
    }
}

#[cfg(test)]
mod tests {
    use crate::Regex;

    #[test]
    fn test_atom_case_sensitive() {
        let regex = Regex::xpath("abc", "").unwrap();
        let op = regex.path("0");
        let matches = regex.matcher("abc").all_matches(op.clone());
        assert_eq!(matches, vec!["abc"]);
        let matches = regex.matcher("a").all_matches(op.clone());
        assert!(matches.is_empty());
        let matches = regex.matcher("abcd").all_matches(op.clone());
        assert_eq!(matches, vec!["abc"]);
    }

    #[test]
    fn test_atom_case_insensitive() {
        let regex = Regex::xpath("abc", "i").unwrap();
        let op = regex.path("0");
        let matches = regex.matcher("abc").all_matches(op.clone());
        assert_eq!(matches, vec!["abc"]);
        let matches = regex.matcher("ABC").all_matches(op.clone());
        assert_eq!(matches, vec!["ABC"]);
    }
}
