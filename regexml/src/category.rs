use std::sync::OnceLock;

use ahash::HashMap;
use ahash::HashMapExt;
use icu_collections::codepointinvlist::CodePointInversionListBuilder;
use icu_properties::maps;
use icu_properties::sets;
use icu_properties::GeneralCategory;
use icu_properties::GeneralCategoryGroup;

use crate::block;
use crate::Error;

fn get_category_group(property: &str) -> Result<GeneralCategoryGroup, Error> {
    // Based on character class escapes in https://www.w3.org/TR/xmlschema-2/#regexs
    Ok(match property {
        // Letters
        "L" => GeneralCategoryGroup::Letter,
        "Lu" => GeneralCategoryGroup::UppercaseLetter,
        "Ll" => GeneralCategoryGroup::LowercaseLetter,
        "Lt" => GeneralCategoryGroup::TitlecaseLetter,
        "Lm" => GeneralCategoryGroup::ModifierLetter,
        "Lo" => GeneralCategoryGroup::OtherLetter,
        // Marks
        "M" => GeneralCategoryGroup::Mark,
        "Mn" => GeneralCategoryGroup::NonspacingMark,
        "Mc" => GeneralCategoryGroup::SpacingMark,
        "Me" => GeneralCategoryGroup::EnclosingMark,
        // Numbers
        "N" => GeneralCategoryGroup::Number,
        "Nd" => GeneralCategoryGroup::DecimalNumber,
        "Nl" => GeneralCategoryGroup::LetterNumber,
        "No" => GeneralCategoryGroup::OtherNumber,
        // Punctuation
        "P" => GeneralCategoryGroup::Punctuation,
        "Pc" => GeneralCategoryGroup::ConnectorPunctuation,
        "Pd" => GeneralCategoryGroup::DashPunctuation,
        "Ps" => GeneralCategoryGroup::OpenPunctuation,
        "Pe" => GeneralCategoryGroup::ClosePunctuation,
        "Pi" => GeneralCategoryGroup::InitialPunctuation,
        "Pf" => GeneralCategoryGroup::FinalPunctuation,
        "Po" => GeneralCategoryGroup::OtherPunctuation,
        // Separators
        "Z" => GeneralCategoryGroup::Separator,
        "Zs" => GeneralCategoryGroup::SpaceSeparator,
        "Zl" => GeneralCategoryGroup::LineSeparator,
        "Zp" => GeneralCategoryGroup::ParagraphSeparator,
        // Symbols
        "S" => GeneralCategoryGroup::Symbol,
        "Sm" => GeneralCategoryGroup::MathSymbol,
        "Sc" => GeneralCategoryGroup::CurrencySymbol,
        "Sk" => GeneralCategoryGroup::ModifierSymbol,
        "So" => GeneralCategoryGroup::OtherSymbol,
        // Other
        "C" => GeneralCategoryGroup::Other,
        "Cc" => GeneralCategoryGroup::Control,
        "Cf" => GeneralCategoryGroup::Format,
        "Co" => GeneralCategoryGroup::PrivateUse,
        "Cn" => GeneralCategoryGroup::Unassigned,
        // Cs is deliberately excluded, as per spec, as Cs do not appear in the
        // character abstraction that XML operates on.
        _ => {
            return Err(Error::syntax(format!(
                "Unknown unicode general category {}",
                property
            )))
        }
    })
}

pub(crate) fn category_group(s: &str) -> Result<CodePointInversionListBuilder, Error> {
    let group = get_category_group(s)?;
    Ok(builder_for_group(group))
}

fn builder_for_group(group: GeneralCategoryGroup) -> CodePointInversionListBuilder {
    let set = sets::for_general_category_group(group);
    let inv_list = set.to_code_point_inversion_list();
    let mut builder = CodePointInversionListBuilder::new();
    builder.add_set(&inv_list);
    builder
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

pub(crate) fn decimal_number() -> CodePointInversionListBuilder {
    let mut builder = CodePointInversionListBuilder::new();
    let s = maps::general_category().get_set_for_value(GeneralCategory::DecimalNumber);
    let il = s.to_code_point_inversion_list();
    builder.add_set(&il);
    builder
}

pub(crate) fn word_char() -> CodePointInversionListBuilder {
    let mut builder = CodePointInversionListBuilder::new();

    // now everything should be in the builder
    builder.complement();

    let punctuation_group = builder_for_group(GeneralCategoryGroup::Punctuation).build();
    let separator_group = builder_for_group(GeneralCategoryGroup::Separator).build();
    let other_group = builder_for_group(GeneralCategoryGroup::Other).build();

    builder.remove_set(&punctuation_group);
    builder.remove_set(&separator_group);
    builder.remove_set(&other_group);

    builder
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
    let lookup = block_lookup();
    let block = lookup.lookup(name)?;
    let mut builder = CodePointInversionListBuilder::new();
    builder.add_range_u32(&(block.start..=block.end));
    Ok(builder)
}
