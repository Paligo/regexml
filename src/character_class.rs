use std::{fmt, rc::Rc};

use ahash::{HashSet, HashSetExt};
// use icu_properties::GeneralCategory;

#[derive(Debug, Clone)]
pub(crate) enum CharacterClass {
    Empty,
    All,
    Inverse(Box<CharacterClass>),
    Char(char),
    CharSet(HashSet<char>),
    // IcuGeneralCategory(GeneralCategory),
}

impl CharacterClass {
    pub(crate) fn from_chars(chars: &[char]) -> Self {
        match chars.len() {
            0 => CharacterClass::Empty,
            1 => CharacterClass::Char(chars[0]),
            _ => CharacterClass::CharSet(HashSet::from_iter(chars.iter().cloned())),
        }
    }

    pub(crate) fn from_set(set: HashSet<char>) -> Self {
        match set.len() {
            0 => CharacterClass::Empty,
            1 => CharacterClass::Char(*set.iter().next().unwrap()),
            _ => CharacterClass::CharSet(set),
        }
    }

    pub(crate) fn complement(self) -> Self {
        match self {
            CharacterClass::Inverse(complement) => *complement,
            CharacterClass::All => CharacterClass::Empty,
            CharacterClass::Empty => CharacterClass::All,
            complement => CharacterClass::Inverse(Box::new(complement)),
        }
    }

    pub(crate) fn union(self, other: CharacterClass) -> Self {
        match (self, other) {
            (CharacterClass::Empty, other) | (other, CharacterClass::Empty) => other,
            (CharacterClass::All, _) | (_, CharacterClass::All) => CharacterClass::All,
            (CharacterClass::Char(a), CharacterClass::Char(b)) => {
                if a == b {
                    CharacterClass::Char(a)
                } else {
                    CharacterClass::CharSet(HashSet::from_iter(vec![a, b]))
                }
            }
            (CharacterClass::CharSet(a), CharacterClass::CharSet(b)) => {
                CharacterClass::CharSet(a.union(&b).copied().collect())
            }
            (CharacterClass::Char(a), CharacterClass::CharSet(b))
            | (CharacterClass::CharSet(b), CharacterClass::Char(a)) => {
                let mut b = b.clone();
                b.insert(a);
                CharacterClass::CharSet(b)
            }
            (CharacterClass::Inverse(a), CharacterClass::Inverse(b)) => {
                CharacterClass::Inverse(Box::new(a.union(*b)))
            }
            (ref original @ CharacterClass::Inverse(ref a), CharacterClass::Char(b))
            | (CharacterClass::Char(b), ref original @ CharacterClass::Inverse(ref a)) => {
                match a.as_ref() {
                    // we can't construct the inverse of all; it's empty
                    CharacterClass::All => unreachable!(),
                    // we can't construct the inverse of empty, it's all
                    CharacterClass::Empty => unreachable!(),
                    // we can't construct the inverse of an inverse, as it's the thing itself
                    CharacterClass::Inverse(_) => unreachable!(),
                    CharacterClass::CharSet(set) => {
                        if set.contains(&b) {
                            // if we have the character in the set (meaning we don't want it), we need to remove it
                            let mut set = set.clone();
                            set.remove(&b);
                            CharacterClass::Inverse(Box::new(CharacterClass::from_set(set)))
                        } else {
                            // if it's not in the set, then we're fine already
                            original.clone()
                        }
                    }
                    CharacterClass::Char(a) => {
                        if *a == b {
                            // we've just removed the character, so the set now matches all
                            CharacterClass::All
                        } else {
                            // it wasn't in the set, so we don't need to do anything
                            original.clone()
                        }
                    }
                }
            }
            (ref original @ CharacterClass::Inverse(ref a), CharacterClass::CharSet(b))
            | (CharacterClass::CharSet(b), ref original @ CharacterClass::Inverse(ref a)) => {
                match a.as_ref() {
                    // we can't construct the inverse of all; it's empty
                    CharacterClass::All => unreachable!(),
                    // we can't construct the inverse of empty, it's all
                    CharacterClass::Empty => unreachable!(),
                    // we can't construct the inverse of an inverse, as it's the thing itself
                    CharacterClass::Inverse(_) => unreachable!(),
                    CharacterClass::CharSet(set) => {
                        // we need to remove all characters given in b from the inverse set,
                        // as it turns out we do want them
                        let mut set = set.clone();
                        let mut any_removed = false;
                        let mut all_removed = true;
                        for c in b {
                            let removed = set.remove(&c);
                            any_removed |= removed;
                            all_removed &= removed;
                        }
                        if all_removed {
                            // we've removed all characters, so the set now matches all
                            CharacterClass::All
                        } else if any_removed {
                            CharacterClass::Inverse(Box::new(CharacterClass::from_set(set)))
                        } else {
                            original.clone()
                        }
                    }
                    CharacterClass::Char(a) => {
                        if b.contains(a) {
                            // we don't want this character, so we now match all
                            CharacterClass::All
                        } else {
                            // it wasn't in the set, so we don't need to do anything
                            original.clone()
                        }
                    }
                }
            }
        }
    }

    pub(crate) fn difference(self, other: CharacterClass) -> Self {
        match (self, other) {
            // substract anything from empty is empty
            (CharacterClass::Empty, _) => CharacterClass::Empty,
            // anything substracting empty is the original thing
            (a, CharacterClass::Empty) => a,
            // if we substract something from all, we get the inverse of that thing
            (CharacterClass::All, b) => CharacterClass::Inverse(Box::new(b)),
            // if we substract all from something, we get the empty
            (a, CharacterClass::All) => CharacterClass::Empty,
            // if we substract a character from the inverse set, we need to add it back
            _ => todo!(),
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

    // pub(crate) fn charset(&self) -> InvertibleCharSet {
    //     match self {
    //         CharacterClass::Empty => InvertibleCharSet::Normal(HashSet::new()),
    //         CharacterClass::Inverse(complement) => {
    //             let charset = complement.charset();
    //             charset.complement()
    //         }
    //         CharacterClass::All => InvertibleCharSet::Normal(HashSet::new()).complement(),
    //         CharacterClass::Char(c) => {
    //             let mut set = HashSet::new();
    //             set.insert(*c);
    //             InvertibleCharSet::Normal(set)
    //         }
    //         CharacterClass::CharSet(set) => InvertibleCharSet::Normal(set.clone()),
    //     }
    // }

    // fn escape_i_lower() -> Self {
    //     CharacterClass::esc
    // }
}

// pub(crate) enum InvertibleCharSet {
//     Inverse(HashSet<char>),
//     Normal(HashSet<char>),
// }

// impl InvertibleCharSet {
//     fn complement(&self) -> Self {
//         match self {
//             InvertibleCharSet::Inverse(set) => InvertibleCharSet::Normal(set.clone()),
//             InvertibleCharSet::Normal(set) => InvertibleCharSet::Inverse(set.clone()),
//         }
//     }

//     fn union(self, other: InvertibleCharSet) -> Self {
//         match (self, other) {
//             (InvertibleCharSet::Normal(a), InvertibleCharSet::Normal(b)) => {
//                 // all the characters in a, and all the characters in b
//                 InvertibleCharSet::Normal(a.union(&b).copied().collect::<HashSet<_>>())
//             }
//             (InvertibleCharSet::Inverse(a), InvertibleCharSet::Inverse(b)) => {
//                 // all the characters not in a, and all the characers not in b,
//                 // so a character has to be not in a, and not in b
//                 InvertibleCharSet::Inverse(a.union(&b).copied().collect::<HashSet<_>>())
//             }
//             (InvertibleCharSet::Inverse(a), InvertibleCharSet::Normal(b)) => {
//                 // all the characters not in a, without the characters also in b,
//                 // as we do want them
//                 InvertibleCharSet::Inverse(a.difference(&b).copied().collect::<HashSet<_>>())
//             }
//             (InvertibleCharSet::Normal(a), InvertibleCharSet::Inverse(b)) => {
//                 // all the characters not in b, without the character also in a,
//                 // as we do want them
//                 InvertibleCharSet::Inverse(b.difference(&a).copied().collect::<HashSet<_>>())
//             }
//         }
//     }

//     fn difference(self, other: InvertibleCharSet) -> Self {
//         match (self, other) {
//             (InvertibleCharSet::Normal(a), InvertibleCharSet::Normal(b)) => {
//                 InvertibleCharSet::Normal(a.difference(&b).copied().collect::<HashSet<_>>())
//             }
//             (InvertibleCharSet::Inverse(a), InvertibleCharSet::Inverse(b)) => {
//                 InvertibleCharSet::Inverse(a.difference(&b).copied().collect::<HashSet<_>>())
//             }
//             (InvertibleCharSet::Inverse(a), InvertibleCharSet::Normal(b)) => {
//                 InvertibleCharSet::Inverse(a.union(&b).copied().collect::<HashSet<_>>())
//             }
//             (InvertibleCharSet::Normal(a), InvertibleCharSet::Inverse(b)) => {
//                 InvertibleCharSet::Inverse(b.union(&a).copied().collect::<HashSet<_>>())
//             }
//         }
//     }
// }

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_chars_no_characters() {
        let cc = CharacterClass::from_chars(&[]);
        assert_eq!(cc, CharacterClass::Empty);
    }

    #[test]
    fn test_from_chars_one_char() {
        let cc = CharacterClass::from_chars(&['a']);
        assert_eq!(cc, CharacterClass::Char('a'));
    }

    #[test]
    fn test_from_chars_multiple_chars() {
        let cc = CharacterClass::from_chars(&['a', 'b', 'c']);
        assert_eq!(cc, CharacterClass::from_chars(&['a', 'b', 'c']));
    }

    #[test]
    fn test_union_empty_with_empty() {
        let cc = CharacterClass::Empty;
        let other = CharacterClass::Empty;
        assert_eq!(cc.union(other), CharacterClass::Empty);
    }

    #[test]
    fn test_union_empty_with_char() {
        let cc = CharacterClass::Empty;
        let other = CharacterClass::Char('a');
        assert_eq!(cc.union(other), CharacterClass::Char('a'));
    }

    #[test]
    fn test_union_char_with_empty() {
        let first = CharacterClass::Char('a');
        let second = CharacterClass::Empty;
        assert_eq!(first.union(second), CharacterClass::Char('a'));
    }

    #[test]
    fn test_union_char_with_char() {
        let first = CharacterClass::Char('a');
        let second = CharacterClass::Char('a');
        assert_eq!(first.union(second), CharacterClass::Char('a'));
    }

    #[test]
    fn test_union_charset_with_charset() {
        let first = CharacterClass::from_chars(&['a', 'b']);
        let second = CharacterClass::from_chars(&['b', 'c']);
        assert_eq!(
            first.union(second),
            CharacterClass::from_chars(&['a', 'b', 'c'])
        );
    }

    #[test]
    fn test_union_char_with_charset() {
        let first = CharacterClass::Char('a');
        let second = CharacterClass::from_chars(&['b', 'c']);
        assert_eq!(
            first.union(second),
            CharacterClass::from_chars(&['a', 'b', 'c'])
        );
    }

    #[test]
    fn test_union_char_with_charset_overlap() {
        let first = CharacterClass::Char('b');
        let second = CharacterClass::from_chars(&['b', 'c']);
        assert_eq!(first.union(second), CharacterClass::from_chars(&['b', 'c']));
    }

    #[test]
    fn test_union_charset_with_char() {
        let first = CharacterClass::from_chars(&['b', 'c']);
        let second = CharacterClass::Char('a');
        assert_eq!(
            first.union(second),
            CharacterClass::from_chars(&['a', 'b', 'c'])
        );
    }

    #[test]
    fn test_union_inverse_char_with_inverse_char() {
        let first = CharacterClass::Char('a').complement();
        let second = CharacterClass::Char('b').complement();
        assert_eq!(
            first.union(second),
            CharacterClass::from_chars(&['a', 'b']).complement()
        );
    }

    #[test]
    fn test_union_inverse_char_with_inverse_char_overlap() {
        let first = CharacterClass::Char('a').complement();
        let second = CharacterClass::Char('a').complement();
        assert_eq!(first.union(second), CharacterClass::Char('a').complement());
    }

    #[test]
    fn test_union_inverse_charset_with_inverse_charset() {
        let first = CharacterClass::from_chars(&['a', 'b']).complement();
        let second = CharacterClass::from_chars(&['b', 'c']).complement();
        assert_eq!(
            first.union(second),
            CharacterClass::from_chars(&['a', 'b', 'c']).complement()
        );
    }

    #[test]
    fn test_union_inverse_char_with_char() {
        // we want all characters except a
        let first = CharacterClass::Char('a').complement();
        // we add b to it, which is already there
        let second = CharacterClass::Char('b');
        assert_eq!(first.union(second), CharacterClass::Char('a').complement());
    }

    #[test]
    fn test_union_inverse_char_with_char_overlap() {
        // we want all characters except a
        let first = CharacterClass::Char('a').complement();
        // we now want a too
        let second = CharacterClass::Char('a');
        assert_eq!(first.union(second), CharacterClass::All);
    }

    #[test]
    fn test_union_inverse_charset_with_char_overlap() {
        // we want all characters except a
        let first = CharacterClass::from_chars(&['a', 'b']).complement();
        // we now want a too
        let second = CharacterClass::Char('a');
        assert_eq!(first.union(second), CharacterClass::Char('b').complement());
    }

    #[test]
    fn test_union_inverse_charset_with_chars_all_overlap() {
        // we want all characters except a and b
        let first = CharacterClass::from_chars(&['a', 'b']).complement();
        // we now want a and b too
        let second = CharacterClass::from_chars(&['a', 'b']);
        assert_eq!(first.union(second), CharacterClass::All);
    }
}
