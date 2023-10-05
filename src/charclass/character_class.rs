use ahash::{HashSet, HashSetExt};

pub(crate) enum CharacterClass {
    Empty,
    Inverse(Box<CharacterClass>),
    Predicate(fn(char) -> bool),
    Char(char),
    CharSet(HashSet<char>),
}

pub(crate) enum InvertableCharSet {
    Inverse(HashSet<char>),
    Normal(HashSet<char>),
}

impl InvertableCharSet {
    fn inverted(&self) -> Self {
        match self {
            InvertableCharSet::Inverse(set) => InvertableCharSet::Normal(set.clone()),
            InvertableCharSet::Normal(set) => InvertableCharSet::Inverse(set.clone()),
        }
    }
}

impl PartialEq for CharacterClass {
    fn eq(&self, other: &CharacterClass) -> bool {
        match (self, other) {
            (CharacterClass::Empty, CharacterClass::Empty) => true,
            (CharacterClass::Inverse(a), CharacterClass::Inverse(b)) => a == b,
            (CharacterClass::Predicate(a), CharacterClass::Predicate(b)) => a == b,
            (CharacterClass::Char(a), CharacterClass::Char(b)) => a == b,
            (CharacterClass::CharSet(a), CharacterClass::CharSet(b)) => a == b,
            _ => false,
        }
    }
}

impl CharacterClass {
    pub(crate) fn test(&self, value: char) -> bool {
        match self {
            CharacterClass::Empty => false,
            CharacterClass::Inverse(complement) => complement.test(value),
            CharacterClass::Predicate(predicate) => predicate(value),
            CharacterClass::Char(c) => value == *c,
            CharacterClass::CharSet(set) => set.contains(&value),
        }
    }

    pub(crate) fn is_disjoint(&self, other: &CharacterClass) -> bool {
        match (self, other) {
            (CharacterClass::Empty, _) => true,
            (_, CharacterClass::Empty) => true,
            (CharacterClass::Inverse(complement), other) => other == complement.as_ref(),
            (self_, CharacterClass::Inverse(complement)) => complement.is_disjoint(self_),
            (CharacterClass::Char(c), other) => !other.test(*c),
            (CharacterClass::CharSet(a), CharacterClass::CharSet(b)) => {
                a.intersection(b).count() == 0
            }
            _ => false,
        }
    }

    pub(crate) fn charset(&self) -> Option<InvertableCharSet> {
        match self {
            CharacterClass::Empty => Some(InvertableCharSet::Normal(HashSet::new())),
            CharacterClass::Inverse(complement) => {
                let charset = complement.charset()?;
                Some(charset.inverted())
            }
            CharacterClass::Predicate(_) => None,
            CharacterClass::Char(c) => {
                let mut set = HashSet::new();
                set.insert(*c);
                Some(InvertableCharSet::Normal(set))
            }
            CharacterClass::CharSet(set) => Some(InvertableCharSet::Normal(set.clone())),
        }
    }
}
