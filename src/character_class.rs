use std::{fmt, rc::Rc};

use ahash::{HashSet, HashSetExt};
// use icu_properties::GeneralCategory;

#[derive(Debug, Clone)]
pub(crate) enum CharacterClass {
    Empty,
    Inverse(Box<CharacterClass>),
    All,
    Char(char),
    CharSet(HashSet<char>),
    // IcuGeneralCategory(GeneralCategory),
}

#[derive(Clone)]
pub(crate) struct PredicateFn(Rc<dyn Fn(char) -> bool>);

impl PredicateFn {
    pub(crate) fn new(f: impl Fn(char) -> bool + 'static) -> Self {
        Self(Rc::new(f))
    }

    fn call(&self, c: char) -> bool {
        (self.0)(c)
    }
}

impl fmt::Debug for PredicateFn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "PredicateFn")
    }
}

impl CharacterClass {
    pub(crate) fn from_chars(chars: &[char]) -> Self {
        match chars.len() {
            0 => CharacterClass::Empty,
            1 => CharacterClass::Char(chars[0]),
            _ => CharacterClass::CharSet(HashSet::from_iter(chars.iter().cloned())),
        }
    }

    pub(crate) fn union(self, other: CharacterClass) -> Self {
        match (self, other) {
            (CharacterClass::Empty, other) => other,
            (s, CharacterClass::Empty) => s,
            (CharacterClass::All, _) => CharacterClass::All,
            (_, CharacterClass::All) => CharacterClass::All,
            (a, b) => {
                let a_charset = a.charset();
                let b_charset = b.charset();

                match a_charset.union(b_charset) {
                    InvertibleCharSet::Normal(a) => CharacterClass::CharSet(a),
                    InvertibleCharSet::Inverse(a) => {
                        CharacterClass::Inverse(Box::new(CharacterClass::CharSet(a)))
                    }
                }
            }
        }
    }

    pub(crate) fn complement(self) -> Self {
        match self {
            CharacterClass::Inverse(complement) => *complement,
            complement => CharacterClass::Inverse(Box::new(complement)),
        }
    }

    pub(crate) fn difference(self, other: CharacterClass) -> Self {
        match (self, other) {
            (CharacterClass::Empty, _) => CharacterClass::Empty,
            (a, CharacterClass::Empty) => a,
            (a, b) => {
                let a_charset = a.charset();
                let b_charset = b.charset();
                match a_charset.difference(b_charset) {
                    InvertibleCharSet::Normal(a) => CharacterClass::CharSet(a),
                    InvertibleCharSet::Inverse(a) => {
                        CharacterClass::Inverse(Box::new(CharacterClass::CharSet(a)))
                    }
                }
            }
        }
    }

    pub(crate) fn escape_s_lower() -> Self {
        CharacterClass::from_chars(&['\t', '\n', '\r', ' '])
    }

    pub(crate) fn escape_s_upper() -> Self {
        CharacterClass::escape_s_lower().complement()
    }

    pub(crate) fn test(&self, value: char) -> bool {
        match self {
            CharacterClass::Empty => false,
            CharacterClass::Inverse(complement) => !complement.test(value),
            CharacterClass::All => true,
            CharacterClass::Char(c) => value == *c,
            CharacterClass::CharSet(set) => set.contains(&value),
            // CharacterClass::IcuGeneralCategory(general_category) => {
            //     icu_properties::maps::general_category()
            //         .get_set_for_value(*general_category)
            //         .as_borrowed()
            //         .contains(value)
            // }
        }
    }

    pub(crate) fn is_disjoint(&self, other: &CharacterClass) -> bool {
        match (self, other) {
            (CharacterClass::Empty, _) => true,
            (_, CharacterClass::Empty) => true,
            (CharacterClass::All, _) => false,
            (_, CharacterClass::All) => false,
            (CharacterClass::Inverse(complement), other) => other == complement.as_ref(),
            (self_, CharacterClass::Inverse(complement)) => complement.is_disjoint(self_),
            (CharacterClass::Char(c), other) => !other.test(*c),
            (CharacterClass::CharSet(a), CharacterClass::CharSet(b)) => {
                a.intersection(b).count() == 0
            }
            _ => false,
        }
    }

    pub(crate) fn charset(&self) -> InvertibleCharSet {
        match self {
            CharacterClass::Empty => InvertibleCharSet::Normal(HashSet::new()),
            CharacterClass::Inverse(complement) => {
                let charset = complement.charset();
                charset.complement()
            }
            CharacterClass::All => InvertibleCharSet::Normal(HashSet::new()).complement(),
            CharacterClass::Char(c) => {
                let mut set = HashSet::new();
                set.insert(*c);
                InvertibleCharSet::Normal(set)
            }
            CharacterClass::CharSet(set) => InvertibleCharSet::Normal(set.clone()),
        }
    }

    // fn escape_i_lower() -> Self {
    //     CharacterClass::esc
    // }
}

pub(crate) enum InvertibleCharSet {
    Inverse(HashSet<char>),
    Normal(HashSet<char>),
}

impl InvertibleCharSet {
    fn complement(&self) -> Self {
        match self {
            InvertibleCharSet::Inverse(set) => InvertibleCharSet::Normal(set.clone()),
            InvertibleCharSet::Normal(set) => InvertibleCharSet::Inverse(set.clone()),
        }
    }

    fn union(self, other: InvertibleCharSet) -> Self {
        match (self, other) {
            (InvertibleCharSet::Normal(a), InvertibleCharSet::Normal(b)) => {
                // all the characters in a, and all the characters in b
                InvertibleCharSet::Normal(a.union(&b).copied().collect::<HashSet<_>>())
            }
            (InvertibleCharSet::Inverse(a), InvertibleCharSet::Inverse(b)) => {
                // all the characters not in a, and all the characers not in b,
                // so a character has to be not in a, and not in b
                InvertibleCharSet::Inverse(a.union(&b).copied().collect::<HashSet<_>>())
            }
            (InvertibleCharSet::Inverse(a), InvertibleCharSet::Normal(b)) => {
                // all the characters not in a, without the characters also in b,
                // as we do want them
                InvertibleCharSet::Inverse(a.difference(&b).copied().collect::<HashSet<_>>())
            }
            (InvertibleCharSet::Normal(a), InvertibleCharSet::Inverse(b)) => {
                // all the characters not in b, without the character also in a,
                // as we do want them
                InvertibleCharSet::Inverse(b.difference(&a).copied().collect::<HashSet<_>>())
            }
        }
    }

    fn difference(self, other: InvertibleCharSet) -> Self {
        match (self, other) {
            (InvertibleCharSet::Normal(a), InvertibleCharSet::Normal(b)) => {
                InvertibleCharSet::Normal(a.difference(&b).copied().collect::<HashSet<_>>())
            }
            (InvertibleCharSet::Inverse(a), InvertibleCharSet::Inverse(b)) => {
                InvertibleCharSet::Inverse(a.difference(&b).copied().collect::<HashSet<_>>())
            }
            (InvertibleCharSet::Inverse(a), InvertibleCharSet::Normal(b)) => {
                InvertibleCharSet::Inverse(a.union(&b).copied().collect::<HashSet<_>>())
            }
            (InvertibleCharSet::Normal(a), InvertibleCharSet::Inverse(b)) => {
                InvertibleCharSet::Inverse(b.union(&a).copied().collect::<HashSet<_>>())
            }
        }
    }
}

impl PartialEq for CharacterClass {
    fn eq(&self, other: &CharacterClass) -> bool {
        match (self, other) {
            (CharacterClass::Empty, CharacterClass::Empty) => true,
            (CharacterClass::Inverse(a), CharacterClass::Inverse(b)) => a == b,
            (CharacterClass::All, CharacterClass::All) => true,
            (CharacterClass::Char(a), CharacterClass::Char(b)) => a == b,
            (CharacterClass::CharSet(a), CharacterClass::CharSet(b)) => a == b,
            _ => false,
        }
    }
}
