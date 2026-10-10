use std::sync::OnceLock;

use ahash::HashMap;
use ahash::HashMapExt;
use icu_collections::codepointinvlist::CodePointInversionList;
use icu_collections::codepointinvlist::CodePointInversionListBuilder;
use icu_properties::sets;
use icu_properties::GeneralCategoryGroup;

use crate::block;
use crate::Error;

// Based on character class escapes in https://www.w3.org/TR/xmlschema-2/#regexs
const CATEGORY_GROUPS: &[(&str, GeneralCategoryGroup)] = &[
    // Letters
    ("L", GeneralCategoryGroup::Letter),
    ("Lu", GeneralCategoryGroup::UppercaseLetter),
    ("Ll", GeneralCategoryGroup::LowercaseLetter),
    ("Lt", GeneralCategoryGroup::TitlecaseLetter),
    ("Lm", GeneralCategoryGroup::ModifierLetter),
    ("Lo", GeneralCategoryGroup::OtherLetter),
    // Marks
    ("M", GeneralCategoryGroup::Mark),
    ("Mn", GeneralCategoryGroup::NonspacingMark),
    ("Mc", GeneralCategoryGroup::SpacingMark),
    ("Me", GeneralCategoryGroup::EnclosingMark),
    // Numbers
    ("N", GeneralCategoryGroup::Number),
    ("Nd", GeneralCategoryGroup::DecimalNumber),
    ("Nl", GeneralCategoryGroup::LetterNumber),
    ("No", GeneralCategoryGroup::OtherNumber),
    // Punctuation
    ("P", GeneralCategoryGroup::Punctuation),
    ("Pc", GeneralCategoryGroup::ConnectorPunctuation),
    ("Pd", GeneralCategoryGroup::DashPunctuation),
    ("Ps", GeneralCategoryGroup::OpenPunctuation),
    ("Pe", GeneralCategoryGroup::ClosePunctuation),
    ("Pi", GeneralCategoryGroup::InitialPunctuation),
    ("Pf", GeneralCategoryGroup::FinalPunctuation),
    ("Po", GeneralCategoryGroup::OtherPunctuation),
    // Separators
    ("Z", GeneralCategoryGroup::Separator),
    ("Zs", GeneralCategoryGroup::SpaceSeparator),
    ("Zl", GeneralCategoryGroup::LineSeparator),
    ("Zp", GeneralCategoryGroup::ParagraphSeparator),
    // Symbols
    ("S", GeneralCategoryGroup::Symbol),
    ("Sm", GeneralCategoryGroup::MathSymbol),
    ("Sc", GeneralCategoryGroup::CurrencySymbol),
    ("Sk", GeneralCategoryGroup::ModifierSymbol),
    ("So", GeneralCategoryGroup::OtherSymbol),
    // Other
    ("C", GeneralCategoryGroup::Other),
    ("Cc", GeneralCategoryGroup::Control),
    ("Cf", GeneralCategoryGroup::Format),
    ("Co", GeneralCategoryGroup::PrivateUse),
    ("Cn", GeneralCategoryGroup::Unassigned),
    // Cs is deliberately excluded, as per spec, as Cs do not appear in the
    // character abstraction that XML operates on.
];

// Building the set of a category from the ICU data walks the whole general
// category trie, which is by far the most expensive part of compiling a
// regular expression that uses `\p{..}`, `\P{..}`, `\d` or `\w`. Each set is
// therefore built only once per process, on first use, and every regular
// expression that needs it starts from a copy.
static CATEGORY_SETS: [OnceLock<CodePointInversionList<'static>>; CATEGORY_GROUPS.len()] =
    [const { OnceLock::new() }; CATEGORY_GROUPS.len()];

static WORD_CHAR_SET: OnceLock<CodePointInversionList<'static>> = OnceLock::new();

fn category_index(property: &str) -> Result<usize, Error> {
    CATEGORY_GROUPS
        .iter()
        .position(|(name, _)| *name == property)
        .ok_or_else(|| Error::syntax(format!("Unknown unicode general category {}", property)))
}

fn category_set(index: usize) -> &'static CodePointInversionList<'static> {
    CATEGORY_SETS[index].get_or_init(|| {
        sets::for_general_category_group(CATEGORY_GROUPS[index].1)
            .to_code_point_inversion_list()
            .into_owned()
    })
}

fn named_category_set(property: &str) -> &'static CodePointInversionList<'static> {
    // only called with the names in `CATEGORY_GROUPS`
    category_set(category_index(property).unwrap())
}

// A fresh builder holding the characters of a cached set, for the compiler to
// combine with others. The symmetric difference with the empty set copies the
// set in one pass, where adding its ranges one by one costs several times more.
fn builder_from(set: &CodePointInversionList) -> CodePointInversionListBuilder {
    let mut builder = CodePointInversionListBuilder::new();
    builder.complement_set(set);
    builder
}

pub(crate) fn category_group(s: &str) -> Result<CodePointInversionListBuilder, Error> {
    let index = category_index(s)?;
    Ok(builder_from(category_set(index)))
}

pub(crate) fn name_start_char() -> CodePointInversionListBuilder {
    let mut builder = CodePointInversionListBuilder::new();
    builder.add_char(':');
    builder.add_char('_');
    builder.add_range(&('A'..='Z'));
    builder.add_range(&('a'..='z'));
    builder.add_range(&('\u{C0}'..='\u{D6}'));
    builder.add_range(&('\u{D8}'..='\u{F6}'));
    builder.add_range(&('\u{F8}'..='\u{2FF}'));
    builder.add_range(&('\u{370}'..='\u{37D}'));
    builder.add_range(&('\u{37F}'..='\u{1FFF}'));
    builder.add_range(&('\u{200C}'..='\u{200D}'));
    builder.add_range(&('\u{2070}'..='\u{218F}'));
    builder.add_range(&('\u{2C00}'..='\u{2FEF}'));
    builder.add_range(&('\u{3001}'..='\u{D7FF}'));
    builder.add_range(&('\u{F900}'..='\u{FDCF}'));
    builder.add_range(&('\u{FDF0}'..='\u{FFFD}'));
    builder.add_range(&('\u{10000}'..='\u{EFFFF}'));
    builder
}

pub(crate) fn name_char() -> CodePointInversionListBuilder {
    let mut builder = name_start_char();
    builder.add_char('-');
    builder.add_char('.');
    builder.add_range(&('0'..='9'));
    builder.add_char('\u{B7}');
    builder.add_range(&('\u{0300}'..='\u{036F}'));
    builder.add_range(&('\u{203F}'..='\u{2040}'));
    builder
}

// `\d` is `\p{Nd}`
pub(crate) fn decimal_number() -> CodePointInversionListBuilder {
    builder_from(named_category_set("Nd"))
}

// `\w` is `[#x0000-#x10FFFF]-[\p{P}\p{Z}\p{C}]`
pub(crate) fn word_char() -> CodePointInversionListBuilder {
    let set = WORD_CHAR_SET.get_or_init(|| {
        let mut builder = CodePointInversionListBuilder::new();
        builder.add_range32(&(0x0000..=0x10FFFF));
        builder.remove_set(named_category_set("P"));
        builder.remove_set(named_category_set("Z"));
        builder.remove_set(named_category_set("C"));
        builder.build()
    });
    builder_from(set)
}

#[derive(Debug)]
pub(crate) struct BlockLookup {
    blocks: HashMap<String, &'static block::Block>,
}

// The XML schema regex spec requires block information
// https://www.w3.org/TR/xmlschema-2/#regexs
impl BlockLookup {
    fn new() -> Self {
        let mut blocks = HashMap::new();
        for block in block::ALL_BLOCKS {
            // In XSD 1.0 we needed to exclude HighSurrogates, LowSurrogates and
            // HighPrivateUseSurrogates blocks. But it appears that the XSD 1.1 specification
            // does not maintain this rule.
            // https://github.com/w3c/qt3tests/issues/61
            // if block.name == "High Surrogates"
            //     || block.name == "Low Surrogates"
            //     || block.name == "High Private Use Surrogates"
            // {
            //     continue;
            // }
            let lookup_name = block.name.replace([' ', '_'], "");
            blocks.insert(lookup_name, block);
        }
        Self { blocks }
    }

    pub(crate) fn lookup(&self, name: &str) -> Result<&'static block::Block, Error> {
        match self.blocks.get(name) {
            Some(block) => Ok(block),
            None => Err(Error::syntax(format!("Unknown Unicode block: {}", name))),
        }
    }
}

static BLOCK_LOOKUP: OnceLock<BlockLookup> = OnceLock::new();

fn block_lookup() -> &'static BlockLookup {
    BLOCK_LOOKUP.get_or_init(BlockLookup::new)
}

pub(crate) fn block(name: &str) -> Result<CodePointInversionListBuilder, Error> {
    // backward compatibility, see XSD 1.1 part 2, section G 4.2.3
    // the other backwards compatibility blocks are handled in the block lookup,
    // but PrivateUse stretches three different ranges, so need special handling
    if name == "PrivateUse" {
        let mut builder = CodePointInversionListBuilder::new();
        builder.add_range32(&(0xE000..=0xF8FF));
        builder.add_range32(&(0xF0000..=0xFFFFD));
        builder.add_range32(&(0x100000..=0x10FFFD));
        return Ok(builder);
    }

    let lookup = block_lookup();
    let block = lookup.lookup(name)?;
    let mut builder = CodePointInversionListBuilder::new();
    builder.add_range32(&(block.start..=block.end));
    Ok(builder)
}

#[cfg(test)]
mod tests {
    use icu_properties::maps;
    use icu_properties::GeneralCategory;

    use super::*;

    fn from_icu(group: GeneralCategoryGroup) -> CodePointInversionList<'static> {
        sets::for_general_category_group(group)
            .to_code_point_inversion_list()
            .into_owned()
    }

    #[test]
    fn test_category_sets_are_the_icu_sets() {
        // the second round is served from the cache
        for _ in 0..2 {
            for (name, group) in CATEGORY_GROUPS {
                assert_eq!(
                    category_group(name).unwrap().build(),
                    from_icu(*group),
                    "{}",
                    name
                );
            }
        }
    }

    #[test]
    fn test_unknown_category() {
        assert_eq!(
            category_group("Cs").err(),
            Some(Error::syntax("Unknown unicode general category Cs"))
        );
    }

    #[test]
    fn test_decimal_number_set() {
        let expected = maps::general_category()
            .get_set_for_value(GeneralCategory::DecimalNumber)
            .to_code_point_inversion_list()
            .into_owned();
        assert_eq!(decimal_number().build(), expected);
        assert_eq!(decimal_number().build(), expected);
    }

    #[test]
    fn test_word_char_set() {
        let mut builder = CodePointInversionListBuilder::new();
        builder.add_range32(&(0x0000..=0x10FFFF));
        builder.remove_set(&from_icu(GeneralCategoryGroup::Punctuation));
        builder.remove_set(&from_icu(GeneralCategoryGroup::Separator));
        builder.remove_set(&from_icu(GeneralCategoryGroup::Other));
        let expected = builder.build();
        assert_eq!(word_char().build(), expected);
        assert_eq!(word_char().build(), expected);
    }

    #[test]
    fn test_builders_do_not_share_the_cached_set() {
        let mut builder = category_group("Lu").unwrap();
        builder.complement();
        builder.add_char('a');
        let changed = builder.build();
        assert!(!changed.contains('A'));
        assert!(changed.contains('a'));

        let fresh = category_group("Lu").unwrap().build();
        assert!(fresh.contains('A'));
        assert!(!fresh.contains('a'));
    }
}
