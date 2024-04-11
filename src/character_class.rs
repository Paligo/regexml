use icu_collections::codepointinvlist::{CodePointInversionList, CodePointInversionListBuilder};

#[derive(Debug, Clone)]
pub(crate) struct CharacterClass(CodePointInversionList<'static>);

impl CharacterClass {
    pub(crate) fn new(code_point_inversion_list: CodePointInversionList<'static>) -> Self {
        Self(code_point_inversion_list)
    }

    pub(crate) fn all() -> Self {
        Self(CodePointInversionList::all())
    }

    pub(crate) fn contains(&self, c: char) -> bool {
        self.0.contains(c)
    }
}

pub(crate) enum CharacterClassBuilder {
    // We could simply use the CodeInversionListBuilder everywhere, but the
    // regex compiler needs single character information, so we handle it
    // separately. Refactoring the regex compiler may allow us to remove this
    // wrinkle.
    Char(char),
    CodePointInversionListBuilder(CodePointInversionListBuilder),
}

impl From<CodePointInversionListBuilder> for CharacterClassBuilder {
    fn from(builder: CodePointInversionListBuilder) -> Self {
        Self::CodePointInversionListBuilder(builder)
    }
}

impl CharacterClassBuilder {
    pub(crate) fn from_char(c: char) -> Self {
        CharacterClassBuilder::Char(c)
    }

    pub(crate) fn from_str(s: &str) -> Self {
        let mut builder = CodePointInversionListBuilder::new();
        for c in s.chars() {
            builder.add_char(c);
        }
        Self::CodePointInversionListBuilder(builder)
    }

    pub(crate) fn complement(self) -> Self {
        match self {
            CharacterClassBuilder::Char(c) => {
                let builder = Self::from_char(c);
                builder.complement()
            }
            CharacterClassBuilder::CodePointInversionListBuilder(mut builder) => {
                builder.complement();
                CharacterClassBuilder::CodePointInversionListBuilder(builder)
            }
        }
    }

    pub(crate) fn union(self, other: Self) -> Self {
        match (self, other) {
            (CharacterClassBuilder::Char(a), CharacterClassBuilder::Char(b)) => {
                let mut builder = CodePointInversionListBuilder::new();
                builder.add_char(a);
                builder.add_char(b);
                CharacterClassBuilder::CodePointInversionListBuilder(builder)
            }
            (
                CharacterClassBuilder::Char(a),
                CharacterClassBuilder::CodePointInversionListBuilder(mut b),
            ) => {
                b.add_char(a);
                CharacterClassBuilder::CodePointInversionListBuilder(b)
            }
            (
                CharacterClassBuilder::CodePointInversionListBuilder(b),
                CharacterClassBuilder::Char(a),
            ) => {
                let a = Self::from_char(a);
                a.union(CharacterClassBuilder::CodePointInversionListBuilder(b))
            }
            (
                CharacterClassBuilder::CodePointInversionListBuilder(mut a),
                CharacterClassBuilder::CodePointInversionListBuilder(b),
            ) => {
                a.add_set(&b.build());
                CharacterClassBuilder::CodePointInversionListBuilder(a)
            }
        }
    }

    pub(crate) fn difference(self, other: Self) -> Self {
        match (self, other) {
            (CharacterClassBuilder::Char(a), CharacterClassBuilder::Char(b)) => {
                if a == b {
                    CharacterClassBuilder::from_str("")
                } else {
                    CharacterClassBuilder::from_char(a)
                }
            }
            (
                CharacterClassBuilder::Char(a),
                CharacterClassBuilder::CodePointInversionListBuilder(b),
            ) => {
                let mut builder = CodePointInversionListBuilder::new();
                builder.add_char(a);
                builder.remove_set(&b.build());
                CharacterClassBuilder::CodePointInversionListBuilder(builder)
            }
            (
                CharacterClassBuilder::CodePointInversionListBuilder(mut a),
                CharacterClassBuilder::Char(b),
            ) => {
                a.remove_char(b);
                CharacterClassBuilder::CodePointInversionListBuilder(a)
            }
            (
                CharacterClassBuilder::CodePointInversionListBuilder(mut a),
                CharacterClassBuilder::CodePointInversionListBuilder(b),
            ) => {
                a.remove_set(&b.build());
                CharacterClassBuilder::CodePointInversionListBuilder(a)
            }
        }
    }

    pub(crate) fn build(self) -> CharacterClass {
        match self {
            CharacterClassBuilder::Char(c) => CharacterClassBuilder::from_char(c).build(),
            CharacterClassBuilder::CodePointInversionListBuilder(builder) => {
                CharacterClass(builder.build())
            }
        }
    }
}
