use std::sync::OnceLock;

use ahash::HashMap;
use ahash::HashMapExt;
use icu_properties::GeneralCategoryGroup;

use crate::block;
use crate::Error;

pub(crate) fn category_group(property: &str) -> Result<GeneralCategoryGroup, Error> {
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

#[derive(Debug)]
pub(crate) struct BlockLookup {
    blocks: HashMap<String, &'static block::Block>,
}

impl BlockLookup {
    fn new() -> Self {
        let mut blocks = HashMap::new();
        for block in block::ALL_BLOCKS {
            //  As per spec, we exclude th eHighSurrogates, LowSurrogates and
            //  HighPrivateUseSurrogates blocks. These blocks identify
            //  "surrogate" characters, which do not occur at the level of the
            //  "character abstraction" that XML instance documents operate on.
            if block.name == "High Surrogates"
                || block.name == "Low Surrogates"
                || block.name == "High Private Use Surrogates"
            {
                continue;
            }
            let lookup_name = block.name.replace(' ', "");
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

pub(crate) fn block_lookup() -> &'static BlockLookup {
    BLOCK_LOOKUP.get_or_init(BlockLookup::new)
}
