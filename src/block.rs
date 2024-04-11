// The XML schema regex spec requires block information
// https://www.w3.org/TR/xmlschema-2/#regexs

// for now, these blocks are hardcoded.

// It would be better if these would be in ICU properties so that they are up
// to date, which I asked about here:
// https://github.com/unicode-org/icu4x/discussions/4798

// I considered the `unicode_blocks` module but it doesn't allow iteration
// through the blocks which is what I require to do the name-based lookup.
pub(crate) struct Block {
    pub(crate) name: &'static str,
    pub(crate) start: u32,
    pub(crate) end: u32,
}

pub(crate) const BASIC_LATIN: Block = Block {
    name: "Basic Latin",
    start: 0x0,
    end: 0x7F,
};

pub(crate) const LATIN_1_SUPPLEMENT: Block = Block {
    name: "Latin-1 Supplement",
    start: 0x80,
    end: 0xFF,
};

pub(crate) const LATIN_EXTENDED_A: Block = Block {
    name: "Latin Extended-A",
    start: 0x100,
    end: 0x17F,
};

pub(crate) const LATIN_EXTENDED_B: Block = Block {
    name: "Latin Extended-B",
    start: 0x180,
    end: 0x24F,
};

pub(crate) const IPA_EXTENSIONS: Block = Block {
    name: "IPA Extensions",
    start: 0x250,
    end: 0x2AF,
};

pub(crate) const SPACING_MODIFIER_LETTERS: Block = Block {
    name: "Spacing Modifier Letters",
    start: 0x2B0,
    end: 0x2FF,
};

pub(crate) const COMBINING_DIACRITICAL_MARKS: Block = Block {
    name: "Combining Diacritical Marks",
    start: 0x300,
    end: 0x36F,
};

pub(crate) const GREEK_AND_COPTIC: Block = Block {
    name: "Greek and Coptic",
    start: 0x370,
    end: 0x3FF,
};

pub(crate) const CYRILLIC: Block = Block {
    name: "Cyrillic",
    start: 0x400,
    end: 0x4FF,
};

pub(crate) const CYRILLIC_SUPPLEMENT: Block = Block {
    name: "Cyrillic Supplement",
    start: 0x500,
    end: 0x52F,
};

pub(crate) const ARMENIAN: Block = Block {
    name: "Armenian",
    start: 0x530,
    end: 0x58F,
};

pub(crate) const HEBREW: Block = Block {
    name: "Hebrew",
    start: 0x590,
    end: 0x5FF,
};

pub(crate) const ARABIC: Block = Block {
    name: "Arabic",
    start: 0x600,
    end: 0x6FF,
};

pub(crate) const SYRIAC: Block = Block {
    name: "Syriac",
    start: 0x700,
    end: 0x74F,
};

pub(crate) const ARABIC_SUPPLEMENT: Block = Block {
    name: "Arabic Supplement",
    start: 0x750,
    end: 0x77F,
};

pub(crate) const THAANA: Block = Block {
    name: "Thaana",
    start: 0x780,
    end: 0x7BF,
};

pub(crate) const NKO: Block = Block {
    name: "NKo",
    start: 0x7C0,
    end: 0x7FF,
};

pub(crate) const SAMARITAN: Block = Block {
    name: "Samaritan",
    start: 0x800,
    end: 0x83F,
};

pub(crate) const MANDAIC: Block = Block {
    name: "Mandaic",
    start: 0x840,
    end: 0x85F,
};

pub(crate) const SYRIAC_SUPPLEMENT: Block = Block {
    name: "Syriac Supplement",
    start: 0x860,
    end: 0x86F,
};

pub(crate) const ARABIC_EXTENDED_B: Block = Block {
    name: "Arabic Extended-B",
    start: 0x870,
    end: 0x89F,
};

pub(crate) const ARABIC_EXTENDED_A: Block = Block {
    name: "Arabic Extended-A",
    start: 0x8A0,
    end: 0x8FF,
};

pub(crate) const DEVANAGARI: Block = Block {
    name: "Devanagari",
    start: 0x900,
    end: 0x97F,
};

pub(crate) const BENGALI: Block = Block {
    name: "Bengali",
    start: 0x980,
    end: 0x9FF,
};

pub(crate) const GURMUKHI: Block = Block {
    name: "Gurmukhi",
    start: 0xA00,
    end: 0xA7F,
};

pub(crate) const GUJARATI: Block = Block {
    name: "Gujarati",
    start: 0xA80,
    end: 0xAFF,
};

pub(crate) const ORIYA: Block = Block {
    name: "Oriya",
    start: 0xB00,
    end: 0xB7F,
};

pub(crate) const TAMIL: Block = Block {
    name: "Tamil",
    start: 0xB80,
    end: 0xBFF,
};

pub(crate) const TELUGU: Block = Block {
    name: "Telugu",
    start: 0xC00,
    end: 0xC7F,
};

pub(crate) const KANNADA: Block = Block {
    name: "Kannada",
    start: 0xC80,
    end: 0xCFF,
};

pub(crate) const MALAYALAM: Block = Block {
    name: "Malayalam",
    start: 0xD00,
    end: 0xD7F,
};

pub(crate) const SINHALA: Block = Block {
    name: "Sinhala",
    start: 0xD80,
    end: 0xDFF,
};

pub(crate) const THAI: Block = Block {
    name: "Thai",
    start: 0xE00,
    end: 0xE7F,
};

pub(crate) const LAO: Block = Block {
    name: "Lao",
    start: 0xE80,
    end: 0xEFF,
};

pub(crate) const TIBETAN: Block = Block {
    name: "Tibetan",
    start: 0xF00,
    end: 0xFFF,
};

pub(crate) const MYANMAR: Block = Block {
    name: "Myanmar",
    start: 0x1000,
    end: 0x109F,
};

pub(crate) const GEORGIAN: Block = Block {
    name: "Georgian",
    start: 0x10A0,
    end: 0x10FF,
};

pub(crate) const HANGUL_JAMO: Block = Block {
    name: "Hangul Jamo",
    start: 0x1100,
    end: 0x11FF,
};

pub(crate) const ETHIOPIC: Block = Block {
    name: "Ethiopic",
    start: 0x1200,
    end: 0x137F,
};

pub(crate) const ETHIOPIC_SUPPLEMENT: Block = Block {
    name: "Ethiopic Supplement",
    start: 0x1380,
    end: 0x139F,
};

pub(crate) const CHEROKEE: Block = Block {
    name: "Cherokee",
    start: 0x13A0,
    end: 0x13FF,
};

pub(crate) const UNIFIED_CANADIAN_ABORIGINAL_SYLLABICS: Block = Block {
    name: "Unified Canadian Aboriginal Syllabics",
    start: 0x1400,
    end: 0x167F,
};

pub(crate) const OGHAM: Block = Block {
    name: "Ogham",
    start: 0x1680,
    end: 0x169F,
};

pub(crate) const RUNIC: Block = Block {
    name: "Runic",
    start: 0x16A0,
    end: 0x16FF,
};

pub(crate) const TAGALOG: Block = Block {
    name: "Tagalog",
    start: 0x1700,
    end: 0x171F,
};

pub(crate) const HANUNOO: Block = Block {
    name: "Hanunoo",
    start: 0x1720,
    end: 0x173F,
};

pub(crate) const BUHID: Block = Block {
    name: "Buhid",
    start: 0x1740,
    end: 0x175F,
};

pub(crate) const TAGBANWA: Block = Block {
    name: "Tagbanwa",
    start: 0x1760,
    end: 0x177F,
};

pub(crate) const KHMER: Block = Block {
    name: "Khmer",
    start: 0x1780,
    end: 0x17FF,
};

pub(crate) const MONGOLIAN: Block = Block {
    name: "Mongolian",
    start: 0x1800,
    end: 0x18AF,
};

pub(crate) const UNIFIED_CANADIAN_ABORIGINAL_SYLLABICS_EXTENDED: Block = Block {
    name: "Unified Canadian Aboriginal Syllabics Extended",
    start: 0x18B0,
    end: 0x18FF,
};

pub(crate) const LIMBU: Block = Block {
    name: "Limbu",
    start: 0x1900,
    end: 0x194F,
};

pub(crate) const TAI_LE: Block = Block {
    name: "Tai Le",
    start: 0x1950,
    end: 0x197F,
};

pub(crate) const NEW_TAI_LUE: Block = Block {
    name: "New Tai Lue",
    start: 0x1980,
    end: 0x19DF,
};

pub(crate) const KHMER_SYMBOLS: Block = Block {
    name: "Khmer Symbols",
    start: 0x19E0,
    end: 0x19FF,
};

pub(crate) const BUGINESE: Block = Block {
    name: "Buginese",
    start: 0x1A00,
    end: 0x1A1F,
};

pub(crate) const TAI_THAM: Block = Block {
    name: "Tai Tham",
    start: 0x1A20,
    end: 0x1AAF,
};

pub(crate) const COMBINING_DIACRITICAL_MARKS_EXTENDED: Block = Block {
    name: "Combining Diacritical Marks Extended",
    start: 0x1AB0,
    end: 0x1AFF,
};

pub(crate) const BALINESE: Block = Block {
    name: "Balinese",
    start: 0x1B00,
    end: 0x1B7F,
};

pub(crate) const SUNDANESE: Block = Block {
    name: "Sundanese",
    start: 0x1B80,
    end: 0x1BBF,
};

pub(crate) const BATAK: Block = Block {
    name: "Batak",
    start: 0x1BC0,
    end: 0x1BFF,
};

pub(crate) const LEPCHA: Block = Block {
    name: "Lepcha",
    start: 0x1C00,
    end: 0x1C4F,
};

pub(crate) const OL_CHIKI: Block = Block {
    name: "Ol Chiki",
    start: 0x1C50,
    end: 0x1C7F,
};

pub(crate) const CYRILLIC_EXTENDED_C: Block = Block {
    name: "Cyrillic Extended-C",
    start: 0x1C80,
    end: 0x1C8F,
};

pub(crate) const GEORGIAN_EXTENDED: Block = Block {
    name: "Georgian Extended",
    start: 0x1C90,
    end: 0x1CBF,
};

pub(crate) const SUNDANESE_SUPPLEMENT: Block = Block {
    name: "Sundanese Supplement",
    start: 0x1CC0,
    end: 0x1CCF,
};

pub(crate) const VEDIC_EXTENSIONS: Block = Block {
    name: "Vedic Extensions",
    start: 0x1CD0,
    end: 0x1CFF,
};

pub(crate) const PHONETIC_EXTENSIONS: Block = Block {
    name: "Phonetic Extensions",
    start: 0x1D00,
    end: 0x1D7F,
};

pub(crate) const PHONETIC_EXTENSIONS_SUPPLEMENT: Block = Block {
    name: "Phonetic Extensions Supplement",
    start: 0x1D80,
    end: 0x1DBF,
};

pub(crate) const COMBINING_DIACRITICAL_MARKS_SUPPLEMENT: Block = Block {
    name: "Combining Diacritical Marks Supplement",
    start: 0x1DC0,
    end: 0x1DFF,
};

pub(crate) const LATIN_EXTENDED_ADDITIONAL: Block = Block {
    name: "Latin Extended Additional",
    start: 0x1E00,
    end: 0x1EFF,
};

pub(crate) const GREEK_EXTENDED: Block = Block {
    name: "Greek Extended",
    start: 0x1F00,
    end: 0x1FFF,
};

pub(crate) const GENERAL_PUNCTUATION: Block = Block {
    name: "General Punctuation",
    start: 0x2000,
    end: 0x206F,
};

pub(crate) const SUPERSCRIPTS_AND_SUBSCRIPTS: Block = Block {
    name: "Superscripts and Subscripts",
    start: 0x2070,
    end: 0x209F,
};

pub(crate) const CURRENCY_SYMBOLS: Block = Block {
    name: "Currency Symbols",
    start: 0x20A0,
    end: 0x20CF,
};

pub(crate) const COMBINING_DIACRITICAL_MARKS_FOR_SYMBOLS: Block = Block {
    name: "Combining Diacritical Marks for Symbols",
    start: 0x20D0,
    end: 0x20FF,
};

pub(crate) const LETTERLIKE_SYMBOLS: Block = Block {
    name: "Letterlike Symbols",
    start: 0x2100,
    end: 0x214F,
};

pub(crate) const NUMBER_FORMS: Block = Block {
    name: "Number Forms",
    start: 0x2150,
    end: 0x218F,
};

pub(crate) const ARROWS: Block = Block {
    name: "Arrows",
    start: 0x2190,
    end: 0x21FF,
};

pub(crate) const MATHEMATICAL_OPERATORS: Block = Block {
    name: "Mathematical Operators",
    start: 0x2200,
    end: 0x22FF,
};

pub(crate) const MISCELLANEOUS_TECHNICAL: Block = Block {
    name: "Miscellaneous Technical",
    start: 0x2300,
    end: 0x23FF,
};

pub(crate) const CONTROL_PICTURES: Block = Block {
    name: "Control Pictures",
    start: 0x2400,
    end: 0x243F,
};

pub(crate) const OPTICAL_CHARACTER_RECOGNITION: Block = Block {
    name: "Optical Character Recognition",
    start: 0x2440,
    end: 0x245F,
};

pub(crate) const ENCLOSED_ALPHANUMERICS: Block = Block {
    name: "Enclosed Alphanumerics",
    start: 0x2460,
    end: 0x24FF,
};

pub(crate) const BOX_DRAWING: Block = Block {
    name: "Box Drawing",
    start: 0x2500,
    end: 0x257F,
};

pub(crate) const BLOCK_ELEMENTS: Block = Block {
    name: "Block Elements",
    start: 0x2580,
    end: 0x259F,
};

pub(crate) const GEOMETRIC_SHAPES: Block = Block {
    name: "Geometric Shapes",
    start: 0x25A0,
    end: 0x25FF,
};

pub(crate) const MISCELLANEOUS_SYMBOLS: Block = Block {
    name: "Miscellaneous Symbols",
    start: 0x2600,
    end: 0x26FF,
};

pub(crate) const DINGBATS: Block = Block {
    name: "Dingbats",
    start: 0x2700,
    end: 0x27BF,
};

pub(crate) const MISCELLANEOUS_MATHEMATICAL_SYMBOLS_A: Block = Block {
    name: "Miscellaneous Mathematical Symbols-A",
    start: 0x27C0,
    end: 0x27EF,
};

pub(crate) const SUPPLEMENTAL_ARROWS_A: Block = Block {
    name: "Supplemental Arrows-A",
    start: 0x27F0,
    end: 0x27FF,
};

pub(crate) const BRAILLE_PATTERNS: Block = Block {
    name: "Braille Patterns",
    start: 0x2800,
    end: 0x28FF,
};

pub(crate) const SUPPLEMENTAL_ARROWS_B: Block = Block {
    name: "Supplemental Arrows-B",
    start: 0x2900,
    end: 0x297F,
};

pub(crate) const MISCELLANEOUS_MATHEMATICAL_SYMBOLS_B: Block = Block {
    name: "Miscellaneous Mathematical Symbols-B",
    start: 0x2980,
    end: 0x29FF,
};

pub(crate) const SUPPLEMENTAL_MATHEMATICAL_OPERATORS: Block = Block {
    name: "Supplemental Mathematical Operators",
    start: 0x2A00,
    end: 0x2AFF,
};

pub(crate) const MISCELLANEOUS_SYMBOLS_AND_ARROWS: Block = Block {
    name: "Miscellaneous Symbols and Arrows",
    start: 0x2B00,
    end: 0x2BFF,
};

pub(crate) const GLAGOLITIC: Block = Block {
    name: "Glagolitic",
    start: 0x2C00,
    end: 0x2C5F,
};

pub(crate) const LATIN_EXTENDED_C: Block = Block {
    name: "Latin Extended-C",
    start: 0x2C60,
    end: 0x2C7F,
};

pub(crate) const COPTIC: Block = Block {
    name: "Coptic",
    start: 0x2C80,
    end: 0x2CFF,
};

pub(crate) const GEORGIAN_SUPPLEMENT: Block = Block {
    name: "Georgian Supplement",
    start: 0x2D00,
    end: 0x2D2F,
};

pub(crate) const TIFINAGH: Block = Block {
    name: "Tifinagh",
    start: 0x2D30,
    end: 0x2D7F,
};

pub(crate) const ETHIOPIC_EXTENDED: Block = Block {
    name: "Ethiopic Extended",
    start: 0x2D80,
    end: 0x2DDF,
};

pub(crate) const CYRILLIC_EXTENDED_A: Block = Block {
    name: "Cyrillic Extended-A",
    start: 0x2DE0,
    end: 0x2DFF,
};

pub(crate) const SUPPLEMENTAL_PUNCTUATION: Block = Block {
    name: "Supplemental Punctuation",
    start: 0x2E00,
    end: 0x2E7F,
};

pub(crate) const CJK_RADICALS_SUPPLEMENT: Block = Block {
    name: "CJK Radicals Supplement",
    start: 0x2E80,
    end: 0x2EFF,
};

pub(crate) const KANGXI_RADICALS: Block = Block {
    name: "Kangxi Radicals",
    start: 0x2F00,
    end: 0x2FDF,
};

pub(crate) const IDEOGRAPHIC_DESCRIPTION_CHARACTERS: Block = Block {
    name: "Ideographic Description Characters",
    start: 0x2FF0,
    end: 0x2FFF,
};

pub(crate) const CJK_SYMBOLS_AND_PUNCTUATION: Block = Block {
    name: "CJK Symbols and Punctuation",
    start: 0x3000,
    end: 0x303F,
};

pub(crate) const HIRAGANA: Block = Block {
    name: "Hiragana",
    start: 0x3040,
    end: 0x309F,
};

pub(crate) const KATAKANA: Block = Block {
    name: "Katakana",
    start: 0x30A0,
    end: 0x30FF,
};

pub(crate) const BOPOMOFO: Block = Block {
    name: "Bopomofo",
    start: 0x3100,
    end: 0x312F,
};

pub(crate) const HANGUL_COMPATIBILITY_JAMO: Block = Block {
    name: "Hangul Compatibility Jamo",
    start: 0x3130,
    end: 0x318F,
};

pub(crate) const KANBUN: Block = Block {
    name: "Kanbun",
    start: 0x3190,
    end: 0x319F,
};

pub(crate) const BOPOMOFO_EXTENDED: Block = Block {
    name: "Bopomofo Extended",
    start: 0x31A0,
    end: 0x31BF,
};

pub(crate) const CJK_STROKES: Block = Block {
    name: "CJK Strokes",
    start: 0x31C0,
    end: 0x31EF,
};

pub(crate) const KATAKANA_PHONETIC_EXTENSIONS: Block = Block {
    name: "Katakana Phonetic Extensions",
    start: 0x31F0,
    end: 0x31FF,
};

pub(crate) const ENCLOSED_CJK_LETTERS_AND_MONTHS: Block = Block {
    name: "Enclosed CJK Letters and Months",
    start: 0x3200,
    end: 0x32FF,
};

pub(crate) const CJK_COMPATIBILITY: Block = Block {
    name: "CJK Compatibility",
    start: 0x3300,
    end: 0x33FF,
};

pub(crate) const CJK_UNIFIED_IDEOGRAPHS_EXTENSION_A: Block = Block {
    name: "CJK Unified Ideographs Extension A",
    start: 0x3400,
    end: 0x4DBF,
};

pub(crate) const YIJING_HEXAGRAM_SYMBOLS: Block = Block {
    name: "Yijing Hexagram Symbols",
    start: 0x4DC0,
    end: 0x4DFF,
};

pub(crate) const CJK_UNIFIED_IDEOGRAPHS: Block = Block {
    name: "CJK Unified Ideographs",
    start: 0x4E00,
    end: 0x9FFF,
};

pub(crate) const YI_SYLLABLES: Block = Block {
    name: "Yi Syllables",
    start: 0xA000,
    end: 0xA48F,
};

pub(crate) const YI_RADICALS: Block = Block {
    name: "Yi Radicals",
    start: 0xA490,
    end: 0xA4CF,
};

pub(crate) const LISU: Block = Block {
    name: "Lisu",
    start: 0xA4D0,
    end: 0xA4FF,
};

pub(crate) const VAI: Block = Block {
    name: "Vai",
    start: 0xA500,
    end: 0xA63F,
};

pub(crate) const CYRILLIC_EXTENDED_B: Block = Block {
    name: "Cyrillic Extended-B",
    start: 0xA640,
    end: 0xA69F,
};

pub(crate) const BAMUM: Block = Block {
    name: "Bamum",
    start: 0xA6A0,
    end: 0xA6FF,
};

pub(crate) const MODIFIER_TONE_LETTERS: Block = Block {
    name: "Modifier Tone Letters",
    start: 0xA700,
    end: 0xA71F,
};

pub(crate) const LATIN_EXTENDED_D: Block = Block {
    name: "Latin Extended-D",
    start: 0xA720,
    end: 0xA7FF,
};

pub(crate) const SYLOTI_NAGRI: Block = Block {
    name: "Syloti Nagri",
    start: 0xA800,
    end: 0xA82F,
};

pub(crate) const COMMON_INDIC_NUMBER_FORMS: Block = Block {
    name: "Common Indic Number Forms",
    start: 0xA830,
    end: 0xA83F,
};

pub(crate) const PHAGS_PA: Block = Block {
    name: "Phags-pa",
    start: 0xA840,
    end: 0xA87F,
};

pub(crate) const SAURASHTRA: Block = Block {
    name: "Saurashtra",
    start: 0xA880,
    end: 0xA8DF,
};

pub(crate) const DEVANAGARI_EXTENDED: Block = Block {
    name: "Devanagari Extended",
    start: 0xA8E0,
    end: 0xA8FF,
};

pub(crate) const KAYAH_LI: Block = Block {
    name: "Kayah Li",
    start: 0xA900,
    end: 0xA92F,
};

pub(crate) const REJANG: Block = Block {
    name: "Rejang",
    start: 0xA930,
    end: 0xA95F,
};

pub(crate) const HANGUL_JAMO_EXTENDED_A: Block = Block {
    name: "Hangul Jamo Extended-A",
    start: 0xA960,
    end: 0xA97F,
};

pub(crate) const JAVANESE: Block = Block {
    name: "Javanese",
    start: 0xA980,
    end: 0xA9DF,
};

pub(crate) const MYANMAR_EXTENDED_B: Block = Block {
    name: "Myanmar Extended-B",
    start: 0xA9E0,
    end: 0xA9FF,
};

pub(crate) const CHAM: Block = Block {
    name: "Cham",
    start: 0xAA00,
    end: 0xAA5F,
};

pub(crate) const MYANMAR_EXTENDED_A: Block = Block {
    name: "Myanmar Extended-A",
    start: 0xAA60,
    end: 0xAA7F,
};

pub(crate) const TAI_VIET: Block = Block {
    name: "Tai Viet",
    start: 0xAA80,
    end: 0xAADF,
};

pub(crate) const MEETEI_MAYEK_EXTENSIONS: Block = Block {
    name: "Meetei Mayek Extensions",
    start: 0xAAE0,
    end: 0xAAFF,
};

pub(crate) const ETHIOPIC_EXTENDED_A: Block = Block {
    name: "Ethiopic Extended-A",
    start: 0xAB00,
    end: 0xAB2F,
};

pub(crate) const LATIN_EXTENDED_E: Block = Block {
    name: "Latin Extended-E",
    start: 0xAB30,
    end: 0xAB6F,
};

pub(crate) const CHEROKEE_SUPPLEMENT: Block = Block {
    name: "Cherokee Supplement",
    start: 0xAB70,
    end: 0xABBF,
};

pub(crate) const MEETEI_MAYEK: Block = Block {
    name: "Meetei Mayek",
    start: 0xABC0,
    end: 0xABFF,
};

pub(crate) const HANGUL_SYLLABLES: Block = Block {
    name: "Hangul Syllables",
    start: 0xAC00,
    end: 0xD7AF,
};

pub(crate) const HANGUL_JAMO_EXTENDED_B: Block = Block {
    name: "Hangul Jamo Extended-B",
    start: 0xD7B0,
    end: 0xD7FF,
};

pub(crate) const HIGH_SURROGATES: Block = Block {
    name: "High Surrogates",
    start: 0xD800,
    end: 0xDB7F,
};

pub(crate) const HIGH_PRIVATE_USE_SURROGATES: Block = Block {
    name: "High Private Use Surrogates",
    start: 0xDB80,
    end: 0xDBFF,
};

pub(crate) const LOW_SURROGATES: Block = Block {
    name: "Low Surrogates",
    start: 0xDC00,
    end: 0xDFFF,
};

pub(crate) const PRIVATE_USE_AREA: Block = Block {
    name: "Private Use Area",
    start: 0xE000,
    end: 0xF8FF,
};

pub(crate) const CJK_COMPATIBILITY_IDEOGRAPHS: Block = Block {
    name: "CJK Compatibility Ideographs",
    start: 0xF900,
    end: 0xFAFF,
};

pub(crate) const ALPHABETIC_PRESENTATION_FORMS: Block = Block {
    name: "Alphabetic Presentation Forms",
    start: 0xFB00,
    end: 0xFB4F,
};

pub(crate) const ARABIC_PRESENTATION_FORMS_A: Block = Block {
    name: "Arabic Presentation Forms-A",
    start: 0xFB50,
    end: 0xFDFF,
};

pub(crate) const VARIATION_SELECTORS: Block = Block {
    name: "Variation Selectors",
    start: 0xFE00,
    end: 0xFE0F,
};

pub(crate) const VERTICAL_FORMS: Block = Block {
    name: "Vertical Forms",
    start: 0xFE10,
    end: 0xFE1F,
};

pub(crate) const COMBINING_HALF_MARKS: Block = Block {
    name: "Combining Half Marks",
    start: 0xFE20,
    end: 0xFE2F,
};

pub(crate) const CJK_COMPATIBILITY_FORMS: Block = Block {
    name: "CJK Compatibility Forms",
    start: 0xFE30,
    end: 0xFE4F,
};

pub(crate) const SMALL_FORM_VARIANTS: Block = Block {
    name: "Small Form Variants",
    start: 0xFE50,
    end: 0xFE6F,
};

pub(crate) const ARABIC_PRESENTATION_FORMS_B: Block = Block {
    name: "Arabic Presentation Forms-B",
    start: 0xFE70,
    end: 0xFEFF,
};

pub(crate) const HALFWIDTH_AND_FULLWIDTH_FORMS: Block = Block {
    name: "Halfwidth and Fullwidth Forms",
    start: 0xFF00,
    end: 0xFFEF,
};

pub(crate) const SPECIALS: Block = Block {
    name: "Specials",
    start: 0xFFF0,
    end: 0xFFFF,
};

pub(crate) const LINEAR_B_SYLLABARY: Block = Block {
    name: "Linear B Syllabary",
    start: 0x10000,
    end: 0x1007F,
};

pub(crate) const LINEAR_B_IDEOGRAMS: Block = Block {
    name: "Linear B Ideograms",
    start: 0x10080,
    end: 0x100FF,
};

pub(crate) const AEGEAN_NUMBERS: Block = Block {
    name: "Aegean Numbers",
    start: 0x10100,
    end: 0x1013F,
};

pub(crate) const ANCIENT_GREEK_NUMBERS: Block = Block {
    name: "Ancient Greek Numbers",
    start: 0x10140,
    end: 0x1018F,
};

pub(crate) const ANCIENT_SYMBOLS: Block = Block {
    name: "Ancient Symbols",
    start: 0x10190,
    end: 0x101CF,
};

pub(crate) const PHAISTOS_DISC: Block = Block {
    name: "Phaistos Disc",
    start: 0x101D0,
    end: 0x101FF,
};

pub(crate) const LYCIAN: Block = Block {
    name: "Lycian",
    start: 0x10280,
    end: 0x1029F,
};

pub(crate) const CARIAN: Block = Block {
    name: "Carian",
    start: 0x102A0,
    end: 0x102DF,
};

pub(crate) const COPTIC_EPACT_NUMBERS: Block = Block {
    name: "Coptic Epact Numbers",
    start: 0x102E0,
    end: 0x102FF,
};

pub(crate) const OLD_ITALIC: Block = Block {
    name: "Old Italic",
    start: 0x10300,
    end: 0x1032F,
};

pub(crate) const GOTHIC: Block = Block {
    name: "Gothic",
    start: 0x10330,
    end: 0x1034F,
};

pub(crate) const OLD_PERMIC: Block = Block {
    name: "Old Permic",
    start: 0x10350,
    end: 0x1037F,
};

pub(crate) const UGARITIC: Block = Block {
    name: "Ugaritic",
    start: 0x10380,
    end: 0x1039F,
};

pub(crate) const OLD_PERSIAN: Block = Block {
    name: "Old Persian",
    start: 0x103A0,
    end: 0x103DF,
};

pub(crate) const DESERET: Block = Block {
    name: "Deseret",
    start: 0x10400,
    end: 0x1044F,
};

pub(crate) const SHAVIAN: Block = Block {
    name: "Shavian",
    start: 0x10450,
    end: 0x1047F,
};

pub(crate) const OSMANYA: Block = Block {
    name: "Osmanya",
    start: 0x10480,
    end: 0x104AF,
};

pub(crate) const OSAGE: Block = Block {
    name: "Osage",
    start: 0x104B0,
    end: 0x104FF,
};

pub(crate) const ELBASAN: Block = Block {
    name: "Elbasan",
    start: 0x10500,
    end: 0x1052F,
};

pub(crate) const CAUCASIAN_ALBANIAN: Block = Block {
    name: "Caucasian Albanian",
    start: 0x10530,
    end: 0x1056F,
};

pub(crate) const VITHKUQI: Block = Block {
    name: "Vithkuqi",
    start: 0x10570,
    end: 0x105BF,
};

pub(crate) const LINEAR_A: Block = Block {
    name: "Linear A",
    start: 0x10600,
    end: 0x1077F,
};

pub(crate) const LATIN_EXTENDED_F: Block = Block {
    name: "Latin Extended-F",
    start: 0x10780,
    end: 0x107BF,
};

pub(crate) const CYPRIOT_SYLLABARY: Block = Block {
    name: "Cypriot Syllabary",
    start: 0x10800,
    end: 0x1083F,
};

pub(crate) const IMPERIAL_ARAMAIC: Block = Block {
    name: "Imperial Aramaic",
    start: 0x10840,
    end: 0x1085F,
};

pub(crate) const PALMYRENE: Block = Block {
    name: "Palmyrene",
    start: 0x10860,
    end: 0x1087F,
};

pub(crate) const NABATAEAN: Block = Block {
    name: "Nabataean",
    start: 0x10880,
    end: 0x108AF,
};

pub(crate) const HATRAN: Block = Block {
    name: "Hatran",
    start: 0x108E0,
    end: 0x108FF,
};

pub(crate) const PHOENICIAN: Block = Block {
    name: "Phoenician",
    start: 0x10900,
    end: 0x1091F,
};

pub(crate) const LYDIAN: Block = Block {
    name: "Lydian",
    start: 0x10920,
    end: 0x1093F,
};

pub(crate) const MEROITIC_HIEROGLYPHS: Block = Block {
    name: "Meroitic Hieroglyphs",
    start: 0x10980,
    end: 0x1099F,
};

pub(crate) const MEROITIC_CURSIVE: Block = Block {
    name: "Meroitic Cursive",
    start: 0x109A0,
    end: 0x109FF,
};

pub(crate) const KHAROSHTHI: Block = Block {
    name: "Kharoshthi",
    start: 0x10A00,
    end: 0x10A5F,
};

pub(crate) const OLD_SOUTH_ARABIAN: Block = Block {
    name: "Old South Arabian",
    start: 0x10A60,
    end: 0x10A7F,
};

pub(crate) const OLD_NORTH_ARABIAN: Block = Block {
    name: "Old North Arabian",
    start: 0x10A80,
    end: 0x10A9F,
};

pub(crate) const MANICHAEAN: Block = Block {
    name: "Manichaean",
    start: 0x10AC0,
    end: 0x10AFF,
};

pub(crate) const AVESTAN: Block = Block {
    name: "Avestan",
    start: 0x10B00,
    end: 0x10B3F,
};

pub(crate) const INSCRIPTIONAL_PARTHIAN: Block = Block {
    name: "Inscriptional Parthian",
    start: 0x10B40,
    end: 0x10B5F,
};

pub(crate) const INSCRIPTIONAL_PAHLAVI: Block = Block {
    name: "Inscriptional Pahlavi",
    start: 0x10B60,
    end: 0x10B7F,
};

pub(crate) const PSALTER_PAHLAVI: Block = Block {
    name: "Psalter Pahlavi",
    start: 0x10B80,
    end: 0x10BAF,
};

pub(crate) const OLD_TURKIC: Block = Block {
    name: "Old Turkic",
    start: 0x10C00,
    end: 0x10C4F,
};

pub(crate) const OLD_HUNGARIAN: Block = Block {
    name: "Old Hungarian",
    start: 0x10C80,
    end: 0x10CFF,
};

pub(crate) const HANIFI_ROHINGYA: Block = Block {
    name: "Hanifi Rohingya",
    start: 0x10D00,
    end: 0x10D3F,
};

pub(crate) const RUMI_NUMERAL_SYMBOLS: Block = Block {
    name: "Rumi Numeral Symbols",
    start: 0x10E60,
    end: 0x10E7F,
};

pub(crate) const YEZIDI: Block = Block {
    name: "Yezidi",
    start: 0x10E80,
    end: 0x10EBF,
};

pub(crate) const ARABIC_EXTENDED_C: Block = Block {
    name: "Arabic Extended-C",
    start: 0x10EC0,
    end: 0x10EFF,
};

pub(crate) const OLD_SOGDIAN: Block = Block {
    name: "Old Sogdian",
    start: 0x10F00,
    end: 0x10F2F,
};

pub(crate) const SOGDIAN: Block = Block {
    name: "Sogdian",
    start: 0x10F30,
    end: 0x10F6F,
};

pub(crate) const OLD_UYGHUR: Block = Block {
    name: "Old Uyghur",
    start: 0x10F70,
    end: 0x10FAF,
};

pub(crate) const CHORASMIAN: Block = Block {
    name: "Chorasmian",
    start: 0x10FB0,
    end: 0x10FDF,
};

pub(crate) const ELYMAIC: Block = Block {
    name: "Elymaic",
    start: 0x10FE0,
    end: 0x10FFF,
};

pub(crate) const BRAHMI: Block = Block {
    name: "Brahmi",
    start: 0x11000,
    end: 0x1107F,
};

pub(crate) const KAITHI: Block = Block {
    name: "Kaithi",
    start: 0x11080,
    end: 0x110CF,
};

pub(crate) const SORA_SOMPENG: Block = Block {
    name: "Sora Sompeng",
    start: 0x110D0,
    end: 0x110FF,
};

pub(crate) const CHAKMA: Block = Block {
    name: "Chakma",
    start: 0x11100,
    end: 0x1114F,
};

pub(crate) const MAHAJANI: Block = Block {
    name: "Mahajani",
    start: 0x11150,
    end: 0x1117F,
};

pub(crate) const SHARADA: Block = Block {
    name: "Sharada",
    start: 0x11180,
    end: 0x111DF,
};

pub(crate) const SINHALA_ARCHAIC_NUMBERS: Block = Block {
    name: "Sinhala Archaic Numbers",
    start: 0x111E0,
    end: 0x111FF,
};

pub(crate) const KHOJKI: Block = Block {
    name: "Khojki",
    start: 0x11200,
    end: 0x1124F,
};

pub(crate) const MULTANI: Block = Block {
    name: "Multani",
    start: 0x11280,
    end: 0x112AF,
};

pub(crate) const KHUDAWADI: Block = Block {
    name: "Khudawadi",
    start: 0x112B0,
    end: 0x112FF,
};

pub(crate) const GRANTHA: Block = Block {
    name: "Grantha",
    start: 0x11300,
    end: 0x1137F,
};

pub(crate) const NEWA: Block = Block {
    name: "Newa",
    start: 0x11400,
    end: 0x1147F,
};

pub(crate) const TIRHUTA: Block = Block {
    name: "Tirhuta",
    start: 0x11480,
    end: 0x114DF,
};

pub(crate) const SIDDHAM: Block = Block {
    name: "Siddham",
    start: 0x11580,
    end: 0x115FF,
};

pub(crate) const MODI: Block = Block {
    name: "Modi",
    start: 0x11600,
    end: 0x1165F,
};

pub(crate) const MONGOLIAN_SUPPLEMENT: Block = Block {
    name: "Mongolian Supplement",
    start: 0x11660,
    end: 0x1167F,
};

pub(crate) const TAKRI: Block = Block {
    name: "Takri",
    start: 0x11680,
    end: 0x116CF,
};

pub(crate) const AHOM: Block = Block {
    name: "Ahom",
    start: 0x11700,
    end: 0x1174F,
};

pub(crate) const DOGRA: Block = Block {
    name: "Dogra",
    start: 0x11800,
    end: 0x1184F,
};

pub(crate) const WARANG_CITI: Block = Block {
    name: "Warang Citi",
    start: 0x118A0,
    end: 0x118FF,
};

pub(crate) const DIVES_AKURU: Block = Block {
    name: "Dives Akuru",
    start: 0x11900,
    end: 0x1195F,
};

pub(crate) const NANDINAGARI: Block = Block {
    name: "Nandinagari",
    start: 0x119A0,
    end: 0x119FF,
};

pub(crate) const ZANABAZAR_SQUARE: Block = Block {
    name: "Zanabazar Square",
    start: 0x11A00,
    end: 0x11A4F,
};

pub(crate) const SOYOMBO: Block = Block {
    name: "Soyombo",
    start: 0x11A50,
    end: 0x11AAF,
};

pub(crate) const UNIFIED_CANADIAN_ABORIGINAL_SYLLABICS_EXTENDED_A: Block = Block {
    name: "Unified Canadian Aboriginal Syllabics Extended-A",
    start: 0x11AB0,
    end: 0x11ABF,
};

pub(crate) const PAU_CIN_HAU: Block = Block {
    name: "Pau Cin Hau",
    start: 0x11AC0,
    end: 0x11AFF,
};

pub(crate) const DEVANAGARI_EXTENDED_A: Block = Block {
    name: "Devanagari Extended-A",
    start: 0x11B00,
    end: 0x11B5F,
};

pub(crate) const BHAIKSUKI: Block = Block {
    name: "Bhaiksuki",
    start: 0x11C00,
    end: 0x11C6F,
};

pub(crate) const MARCHEN: Block = Block {
    name: "Marchen",
    start: 0x11C70,
    end: 0x11CBF,
};

pub(crate) const MASARAM_GONDI: Block = Block {
    name: "Masaram Gondi",
    start: 0x11D00,
    end: 0x11D5F,
};

pub(crate) const GUNJALA_GONDI: Block = Block {
    name: "Gunjala Gondi",
    start: 0x11D60,
    end: 0x11DAF,
};

pub(crate) const MAKASAR: Block = Block {
    name: "Makasar",
    start: 0x11EE0,
    end: 0x11EFF,
};

pub(crate) const KAWI: Block = Block {
    name: "Kawi",
    start: 0x11F00,
    end: 0x11F5F,
};

pub(crate) const LISU_SUPPLEMENT: Block = Block {
    name: "Lisu Supplement",
    start: 0x11FB0,
    end: 0x11FBF,
};

pub(crate) const TAMIL_SUPPLEMENT: Block = Block {
    name: "Tamil Supplement",
    start: 0x11FC0,
    end: 0x11FFF,
};

pub(crate) const CUNEIFORM: Block = Block {
    name: "Cuneiform",
    start: 0x12000,
    end: 0x123FF,
};

pub(crate) const CUNEIFORM_NUMBERS_AND_PUNCTUATION: Block = Block {
    name: "Cuneiform Numbers and Punctuation",
    start: 0x12400,
    end: 0x1247F,
};

pub(crate) const EARLY_DYNASTIC_CUNEIFORM: Block = Block {
    name: "Early Dynastic Cuneiform",
    start: 0x12480,
    end: 0x1254F,
};

pub(crate) const CYPRO_MINOAN: Block = Block {
    name: "Cypro-Minoan",
    start: 0x12F90,
    end: 0x12FFF,
};

pub(crate) const EGYPTIAN_HIEROGLYPHS: Block = Block {
    name: "Egyptian Hieroglyphs",
    start: 0x13000,
    end: 0x1342F,
};

pub(crate) const EGYPTIAN_HIEROGLYPH_FORMAT_CONTROLS: Block = Block {
    name: "Egyptian Hieroglyph Format Controls",
    start: 0x13430,
    end: 0x1345F,
};

pub(crate) const ANATOLIAN_HIEROGLYPHS: Block = Block {
    name: "Anatolian Hieroglyphs",
    start: 0x14400,
    end: 0x1467F,
};

pub(crate) const BAMUM_SUPPLEMENT: Block = Block {
    name: "Bamum Supplement",
    start: 0x16800,
    end: 0x16A3F,
};

pub(crate) const MRO: Block = Block {
    name: "Mro",
    start: 0x16A40,
    end: 0x16A6F,
};

pub(crate) const TANGSA: Block = Block {
    name: "Tangsa",
    start: 0x16A70,
    end: 0x16ACF,
};

pub(crate) const BASSA_VAH: Block = Block {
    name: "Bassa Vah",
    start: 0x16AD0,
    end: 0x16AFF,
};

pub(crate) const PAHAWH_HMONG: Block = Block {
    name: "Pahawh Hmong",
    start: 0x16B00,
    end: 0x16B8F,
};

pub(crate) const MEDEFAIDRIN: Block = Block {
    name: "Medefaidrin",
    start: 0x16E40,
    end: 0x16E9F,
};

pub(crate) const MIAO: Block = Block {
    name: "Miao",
    start: 0x16F00,
    end: 0x16F9F,
};

pub(crate) const IDEOGRAPHIC_SYMBOLS_AND_PUNCTUATION: Block = Block {
    name: "Ideographic Symbols and Punctuation",
    start: 0x16FE0,
    end: 0x16FFF,
};

pub(crate) const TANGUT: Block = Block {
    name: "Tangut",
    start: 0x17000,
    end: 0x187FF,
};

pub(crate) const TANGUT_COMPONENTS: Block = Block {
    name: "Tangut Components",
    start: 0x18800,
    end: 0x18AFF,
};

pub(crate) const KHITAN_SMALL_SCRIPT: Block = Block {
    name: "Khitan Small Script",
    start: 0x18B00,
    end: 0x18CFF,
};

pub(crate) const TANGUT_SUPPLEMENT: Block = Block {
    name: "Tangut Supplement",
    start: 0x18D00,
    end: 0x18D7F,
};

pub(crate) const KANA_EXTENDED_B: Block = Block {
    name: "Kana Extended-B",
    start: 0x1AFF0,
    end: 0x1AFFF,
};

pub(crate) const KANA_SUPPLEMENT: Block = Block {
    name: "Kana Supplement",
    start: 0x1B000,
    end: 0x1B0FF,
};

pub(crate) const KANA_EXTENDED_A: Block = Block {
    name: "Kana Extended-A",
    start: 0x1B100,
    end: 0x1B12F,
};

pub(crate) const SMALL_KANA_EXTENSION: Block = Block {
    name: "Small Kana Extension",
    start: 0x1B130,
    end: 0x1B16F,
};

pub(crate) const NUSHU: Block = Block {
    name: "Nushu",
    start: 0x1B170,
    end: 0x1B2FF,
};

pub(crate) const DUPLOYAN: Block = Block {
    name: "Duployan",
    start: 0x1BC00,
    end: 0x1BC9F,
};

pub(crate) const SHORTHAND_FORMAT_CONTROLS: Block = Block {
    name: "Shorthand Format Controls",
    start: 0x1BCA0,
    end: 0x1BCAF,
};

pub(crate) const ZNAMENNY_MUSICAL_NOTATION: Block = Block {
    name: "Znamenny Musical Notation",
    start: 0x1CF00,
    end: 0x1CFCF,
};

pub(crate) const BYZANTINE_MUSICAL_SYMBOLS: Block = Block {
    name: "Byzantine Musical Symbols",
    start: 0x1D000,
    end: 0x1D0FF,
};

pub(crate) const MUSICAL_SYMBOLS: Block = Block {
    name: "Musical Symbols",
    start: 0x1D100,
    end: 0x1D1FF,
};

pub(crate) const ANCIENT_GREEK_MUSICAL_NOTATION: Block = Block {
    name: "Ancient Greek Musical Notation",
    start: 0x1D200,
    end: 0x1D24F,
};

pub(crate) const KAKTOVIK_NUMERALS: Block = Block {
    name: "Kaktovik Numerals",
    start: 0x1D2C0,
    end: 0x1D2DF,
};

pub(crate) const MAYAN_NUMERALS: Block = Block {
    name: "Mayan Numerals",
    start: 0x1D2E0,
    end: 0x1D2FF,
};

pub(crate) const TAI_XUAN_JING_SYMBOLS: Block = Block {
    name: "Tai Xuan Jing Symbols",
    start: 0x1D300,
    end: 0x1D35F,
};

pub(crate) const COUNTING_ROD_NUMERALS: Block = Block {
    name: "Counting Rod Numerals",
    start: 0x1D360,
    end: 0x1D37F,
};

pub(crate) const MATHEMATICAL_ALPHANUMERIC_SYMBOLS: Block = Block {
    name: "Mathematical Alphanumeric Symbols",
    start: 0x1D400,
    end: 0x1D7FF,
};

pub(crate) const SUTTON_SIGNWRITING: Block = Block {
    name: "Sutton SignWriting",
    start: 0x1D800,
    end: 0x1DAAF,
};

pub(crate) const LATIN_EXTENDED_G: Block = Block {
    name: "Latin Extended-G",
    start: 0x1DF00,
    end: 0x1DFFF,
};

pub(crate) const GLAGOLITIC_SUPPLEMENT: Block = Block {
    name: "Glagolitic Supplement",
    start: 0x1E000,
    end: 0x1E02F,
};

pub(crate) const CYRILLIC_EXTENDED_D: Block = Block {
    name: "Cyrillic Extended-D",
    start: 0x1E030,
    end: 0x1E08F,
};

pub(crate) const NYIAKENG_PUACHUE_HMONG: Block = Block {
    name: "Nyiakeng Puachue Hmong",
    start: 0x1E100,
    end: 0x1E14F,
};

pub(crate) const TOTO: Block = Block {
    name: "Toto",
    start: 0x1E290,
    end: 0x1E2BF,
};

pub(crate) const WANCHO: Block = Block {
    name: "Wancho",
    start: 0x1E2C0,
    end: 0x1E2FF,
};

pub(crate) const NAG_MUNDARI: Block = Block {
    name: "Nag Mundari",
    start: 0x1E4D0,
    end: 0x1E4FF,
};

pub(crate) const ETHIOPIC_EXTENDED_B: Block = Block {
    name: "Ethiopic Extended-B",
    start: 0x1E7E0,
    end: 0x1E7FF,
};

pub(crate) const MENDE_KIKAKUI: Block = Block {
    name: "Mende Kikakui",
    start: 0x1E800,
    end: 0x1E8DF,
};

pub(crate) const ADLAM: Block = Block {
    name: "Adlam",
    start: 0x1E900,
    end: 0x1E95F,
};

pub(crate) const INDIC_SIYAQ_NUMBERS: Block = Block {
    name: "Indic Siyaq Numbers",
    start: 0x1EC70,
    end: 0x1ECBF,
};

pub(crate) const OTTOMAN_SIYAQ_NUMBERS: Block = Block {
    name: "Ottoman Siyaq Numbers",
    start: 0x1ED00,
    end: 0x1ED4F,
};

pub(crate) const ARABIC_MATHEMATICAL_ALPHABETIC_SYMBOLS: Block = Block {
    name: "Arabic Mathematical Alphabetic Symbols",
    start: 0x1EE00,
    end: 0x1EEFF,
};

pub(crate) const MAHJONG_TILES: Block = Block {
    name: "Mahjong Tiles",
    start: 0x1F000,
    end: 0x1F02F,
};

pub(crate) const DOMINO_TILES: Block = Block {
    name: "Domino Tiles",
    start: 0x1F030,
    end: 0x1F09F,
};

pub(crate) const PLAYING_CARDS: Block = Block {
    name: "Playing Cards",
    start: 0x1F0A0,
    end: 0x1F0FF,
};

pub(crate) const ENCLOSED_ALPHANUMERIC_SUPPLEMENT: Block = Block {
    name: "Enclosed Alphanumeric Supplement",
    start: 0x1F100,
    end: 0x1F1FF,
};

pub(crate) const ENCLOSED_IDEOGRAPHIC_SUPPLEMENT: Block = Block {
    name: "Enclosed Ideographic Supplement",
    start: 0x1F200,
    end: 0x1F2FF,
};

pub(crate) const MISCELLANEOUS_SYMBOLS_AND_PICTOGRAPHS: Block = Block {
    name: "Miscellaneous Symbols and Pictographs",
    start: 0x1F300,
    end: 0x1F5FF,
};

pub(crate) const EMOTICONS: Block = Block {
    name: "Emoticons",
    start: 0x1F600,
    end: 0x1F64F,
};

pub(crate) const ORNAMENTAL_DINGBATS: Block = Block {
    name: "Ornamental Dingbats",
    start: 0x1F650,
    end: 0x1F67F,
};

pub(crate) const TRANSPORT_AND_MAP_SYMBOLS: Block = Block {
    name: "Transport and Map Symbols",
    start: 0x1F680,
    end: 0x1F6FF,
};

pub(crate) const ALCHEMICAL_SYMBOLS: Block = Block {
    name: "Alchemical Symbols",
    start: 0x1F700,
    end: 0x1F77F,
};

pub(crate) const GEOMETRIC_SHAPES_EXTENDED: Block = Block {
    name: "Geometric Shapes Extended",
    start: 0x1F780,
    end: 0x1F7FF,
};

pub(crate) const SUPPLEMENTAL_ARROWS_C: Block = Block {
    name: "Supplemental Arrows-C",
    start: 0x1F800,
    end: 0x1F8FF,
};

pub(crate) const SUPPLEMENTAL_SYMBOLS_AND_PICTOGRAPHS: Block = Block {
    name: "Supplemental Symbols and Pictographs",
    start: 0x1F900,
    end: 0x1F9FF,
};

pub(crate) const CHESS_SYMBOLS: Block = Block {
    name: "Chess Symbols",
    start: 0x1FA00,
    end: 0x1FA6F,
};

pub(crate) const SYMBOLS_AND_PICTOGRAPHS_EXTENDED_A: Block = Block {
    name: "Symbols and Pictographs Extended-A",
    start: 0x1FA70,
    end: 0x1FAFF,
};

pub(crate) const SYMBOLS_FOR_LEGACY_COMPUTING: Block = Block {
    name: "Symbols for Legacy Computing",
    start: 0x1FB00,
    end: 0x1FBFF,
};

pub(crate) const CJK_UNIFIED_IDEOGRAPHS_EXTENSION_B: Block = Block {
    name: "CJK Unified Ideographs Extension B",
    start: 0x20000,
    end: 0x2A6DF,
};

pub(crate) const CJK_UNIFIED_IDEOGRAPHS_EXTENSION_C: Block = Block {
    name: "CJK Unified Ideographs Extension C",
    start: 0x2A700,
    end: 0x2B73F,
};

pub(crate) const CJK_UNIFIED_IDEOGRAPHS_EXTENSION_D: Block = Block {
    name: "CJK Unified Ideographs Extension D",
    start: 0x2B740,
    end: 0x2B81F,
};

pub(crate) const CJK_UNIFIED_IDEOGRAPHS_EXTENSION_E: Block = Block {
    name: "CJK Unified Ideographs Extension E",
    start: 0x2B820,
    end: 0x2CEAF,
};

pub(crate) const CJK_UNIFIED_IDEOGRAPHS_EXTENSION_F: Block = Block {
    name: "CJK Unified Ideographs Extension F",
    start: 0x2CEB0,
    end: 0x2EBEF,
};

pub(crate) const CJK_UNIFIED_IDEOGRAPHS_EXTENSION_I: Block = Block {
    name: "CJK Unified Ideographs Extension I",
    start: 0x2EBF0,
    end: 0x2EE5F,
};

pub(crate) const CJK_COMPATIBILITY_IDEOGRAPHS_SUPPLEMENT: Block = Block {
    name: "CJK Compatibility Ideographs Supplement",
    start: 0x2F800,
    end: 0x2FA1F,
};

pub(crate) const CJK_UNIFIED_IDEOGRAPHS_EXTENSION_G: Block = Block {
    name: "CJK Unified Ideographs Extension G",
    start: 0x30000,
    end: 0x3134F,
};

pub(crate) const CJK_UNIFIED_IDEOGRAPHS_EXTENSION_H: Block = Block {
    name: "CJK Unified Ideographs Extension H",
    start: 0x31350,
    end: 0x323AF,
};

pub(crate) const TAGS: Block = Block {
    name: "Tags",
    start: 0xE0000,
    end: 0xE007F,
};

pub(crate) const VARIATION_SELECTORS_SUPPLEMENT: Block = Block {
    name: "Variation Selectors Supplement",
    start: 0xE0100,
    end: 0xE01EF,
};

pub(crate) const SUPPLEMENTARY_PRIVATE_USE_AREA_A: Block = Block {
    name: "Supplementary Private Use Area-A",
    start: 0xF0000,
    end: 0xFFFFF,
};

pub(crate) const SUPPLEMENTARY_PRIVATE_USE_AREA_B: Block = Block {
    name: "Supplementary Private Use Area-B",
    start: 0x100000,
    end: 0x10FFFF,
};

pub(crate) const ALL_BLOCKS: &[Block] = &[
    BASIC_LATIN,
    LATIN_1_SUPPLEMENT,
    LATIN_EXTENDED_A,
    LATIN_EXTENDED_B,
    IPA_EXTENSIONS,
    SPACING_MODIFIER_LETTERS,
    COMBINING_DIACRITICAL_MARKS,
    GREEK_AND_COPTIC,
    CYRILLIC,
    CYRILLIC_SUPPLEMENT,
    ARMENIAN,
    HEBREW,
    ARABIC,
    SYRIAC,
    ARABIC_SUPPLEMENT,
    THAANA,
    NKO,
    SAMARITAN,
    MANDAIC,
    SYRIAC_SUPPLEMENT,
    ARABIC_EXTENDED_B,
    ARABIC_EXTENDED_A,
    DEVANAGARI,
    BENGALI,
    GURMUKHI,
    GUJARATI,
    ORIYA,
    TAMIL,
    TELUGU,
    KANNADA,
    MALAYALAM,
    SINHALA,
    THAI,
    LAO,
    TIBETAN,
    MYANMAR,
    GEORGIAN,
    HANGUL_JAMO,
    ETHIOPIC,
    ETHIOPIC_SUPPLEMENT,
    CHEROKEE,
    UNIFIED_CANADIAN_ABORIGINAL_SYLLABICS,
    OGHAM,
    RUNIC,
    TAGALOG,
    HANUNOO,
    BUHID,
    TAGBANWA,
    KHMER,
    MONGOLIAN,
    UNIFIED_CANADIAN_ABORIGINAL_SYLLABICS_EXTENDED,
    LIMBU,
    TAI_LE,
    NEW_TAI_LUE,
    KHMER_SYMBOLS,
    BUGINESE,
    TAI_THAM,
    COMBINING_DIACRITICAL_MARKS_EXTENDED,
    BALINESE,
    SUNDANESE,
    BATAK,
    LEPCHA,
    OL_CHIKI,
    CYRILLIC_EXTENDED_C,
    GEORGIAN_EXTENDED,
    SUNDANESE_SUPPLEMENT,
    VEDIC_EXTENSIONS,
    PHONETIC_EXTENSIONS,
    PHONETIC_EXTENSIONS_SUPPLEMENT,
    COMBINING_DIACRITICAL_MARKS_SUPPLEMENT,
    LATIN_EXTENDED_ADDITIONAL,
    GREEK_EXTENDED,
    GENERAL_PUNCTUATION,
    SUPERSCRIPTS_AND_SUBSCRIPTS,
    CURRENCY_SYMBOLS,
    COMBINING_DIACRITICAL_MARKS_FOR_SYMBOLS,
    LETTERLIKE_SYMBOLS,
    NUMBER_FORMS,
    ARROWS,
    MATHEMATICAL_OPERATORS,
    MISCELLANEOUS_TECHNICAL,
    CONTROL_PICTURES,
    OPTICAL_CHARACTER_RECOGNITION,
    ENCLOSED_ALPHANUMERICS,
    BOX_DRAWING,
    BLOCK_ELEMENTS,
    GEOMETRIC_SHAPES,
    MISCELLANEOUS_SYMBOLS,
    DINGBATS,
    MISCELLANEOUS_MATHEMATICAL_SYMBOLS_A,
    SUPPLEMENTAL_ARROWS_A,
    BRAILLE_PATTERNS,
    SUPPLEMENTAL_ARROWS_B,
    MISCELLANEOUS_MATHEMATICAL_SYMBOLS_B,
    SUPPLEMENTAL_MATHEMATICAL_OPERATORS,
    MISCELLANEOUS_SYMBOLS_AND_ARROWS,
    GLAGOLITIC,
    LATIN_EXTENDED_C,
    COPTIC,
    GEORGIAN_SUPPLEMENT,
    TIFINAGH,
    ETHIOPIC_EXTENDED,
    CYRILLIC_EXTENDED_A,
    SUPPLEMENTAL_PUNCTUATION,
    CJK_RADICALS_SUPPLEMENT,
    KANGXI_RADICALS,
    IDEOGRAPHIC_DESCRIPTION_CHARACTERS,
    CJK_SYMBOLS_AND_PUNCTUATION,
    HIRAGANA,
    KATAKANA,
    BOPOMOFO,
    HANGUL_COMPATIBILITY_JAMO,
    KANBUN,
    BOPOMOFO_EXTENDED,
    CJK_STROKES,
    KATAKANA_PHONETIC_EXTENSIONS,
    ENCLOSED_CJK_LETTERS_AND_MONTHS,
    CJK_COMPATIBILITY,
    CJK_UNIFIED_IDEOGRAPHS_EXTENSION_A,
    YIJING_HEXAGRAM_SYMBOLS,
    CJK_UNIFIED_IDEOGRAPHS,
    YI_SYLLABLES,
    YI_RADICALS,
    LISU,
    VAI,
    CYRILLIC_EXTENDED_B,
    BAMUM,
    MODIFIER_TONE_LETTERS,
    LATIN_EXTENDED_D,
    SYLOTI_NAGRI,
    COMMON_INDIC_NUMBER_FORMS,
    PHAGS_PA,
    SAURASHTRA,
    DEVANAGARI_EXTENDED,
    KAYAH_LI,
    REJANG,
    HANGUL_JAMO_EXTENDED_A,
    JAVANESE,
    MYANMAR_EXTENDED_B,
    CHAM,
    MYANMAR_EXTENDED_A,
    TAI_VIET,
    MEETEI_MAYEK_EXTENSIONS,
    ETHIOPIC_EXTENDED_A,
    LATIN_EXTENDED_E,
    CHEROKEE_SUPPLEMENT,
    MEETEI_MAYEK,
    HANGUL_SYLLABLES,
    HANGUL_JAMO_EXTENDED_B,
    HIGH_SURROGATES,
    HIGH_PRIVATE_USE_SURROGATES,
    LOW_SURROGATES,
    PRIVATE_USE_AREA,
    CJK_COMPATIBILITY_IDEOGRAPHS,
    ALPHABETIC_PRESENTATION_FORMS,
    ARABIC_PRESENTATION_FORMS_A,
    VARIATION_SELECTORS,
    VERTICAL_FORMS,
    COMBINING_HALF_MARKS,
    CJK_COMPATIBILITY_FORMS,
    SMALL_FORM_VARIANTS,
    ARABIC_PRESENTATION_FORMS_B,
    HALFWIDTH_AND_FULLWIDTH_FORMS,
    SPECIALS,
    LINEAR_B_SYLLABARY,
    LINEAR_B_IDEOGRAMS,
    AEGEAN_NUMBERS,
    ANCIENT_GREEK_NUMBERS,
    ANCIENT_SYMBOLS,
    PHAISTOS_DISC,
    LYCIAN,
    CARIAN,
    COPTIC_EPACT_NUMBERS,
    OLD_ITALIC,
    GOTHIC,
    OLD_PERMIC,
    UGARITIC,
    OLD_PERSIAN,
    DESERET,
    SHAVIAN,
    OSMANYA,
    OSAGE,
    ELBASAN,
    CAUCASIAN_ALBANIAN,
    VITHKUQI,
    LINEAR_A,
    LATIN_EXTENDED_F,
    CYPRIOT_SYLLABARY,
    IMPERIAL_ARAMAIC,
    PALMYRENE,
    NABATAEAN,
    HATRAN,
    PHOENICIAN,
    LYDIAN,
    MEROITIC_HIEROGLYPHS,
    MEROITIC_CURSIVE,
    KHAROSHTHI,
    OLD_SOUTH_ARABIAN,
    OLD_NORTH_ARABIAN,
    MANICHAEAN,
    AVESTAN,
    INSCRIPTIONAL_PARTHIAN,
    INSCRIPTIONAL_PAHLAVI,
    PSALTER_PAHLAVI,
    OLD_TURKIC,
    OLD_HUNGARIAN,
    HANIFI_ROHINGYA,
    RUMI_NUMERAL_SYMBOLS,
    YEZIDI,
    ARABIC_EXTENDED_C,
    OLD_SOGDIAN,
    SOGDIAN,
    OLD_UYGHUR,
    CHORASMIAN,
    ELYMAIC,
    BRAHMI,
    KAITHI,
    SORA_SOMPENG,
    CHAKMA,
    MAHAJANI,
    SHARADA,
    SINHALA_ARCHAIC_NUMBERS,
    KHOJKI,
    MULTANI,
    KHUDAWADI,
    GRANTHA,
    NEWA,
    TIRHUTA,
    SIDDHAM,
    MODI,
    MONGOLIAN_SUPPLEMENT,
    TAKRI,
    AHOM,
    DOGRA,
    WARANG_CITI,
    DIVES_AKURU,
    NANDINAGARI,
    ZANABAZAR_SQUARE,
    SOYOMBO,
    UNIFIED_CANADIAN_ABORIGINAL_SYLLABICS_EXTENDED_A,
    PAU_CIN_HAU,
    DEVANAGARI_EXTENDED_A,
    BHAIKSUKI,
    MARCHEN,
    MASARAM_GONDI,
    GUNJALA_GONDI,
    MAKASAR,
    KAWI,
    LISU_SUPPLEMENT,
    TAMIL_SUPPLEMENT,
    CUNEIFORM,
    CUNEIFORM_NUMBERS_AND_PUNCTUATION,
    EARLY_DYNASTIC_CUNEIFORM,
    CYPRO_MINOAN,
    EGYPTIAN_HIEROGLYPHS,
    EGYPTIAN_HIEROGLYPH_FORMAT_CONTROLS,
    ANATOLIAN_HIEROGLYPHS,
    BAMUM_SUPPLEMENT,
    MRO,
    TANGSA,
    BASSA_VAH,
    PAHAWH_HMONG,
    MEDEFAIDRIN,
    MIAO,
    IDEOGRAPHIC_SYMBOLS_AND_PUNCTUATION,
    TANGUT,
    TANGUT_COMPONENTS,
    KHITAN_SMALL_SCRIPT,
    TANGUT_SUPPLEMENT,
    KANA_EXTENDED_B,
    KANA_SUPPLEMENT,
    KANA_EXTENDED_A,
    SMALL_KANA_EXTENSION,
    NUSHU,
    DUPLOYAN,
    SHORTHAND_FORMAT_CONTROLS,
    ZNAMENNY_MUSICAL_NOTATION,
    BYZANTINE_MUSICAL_SYMBOLS,
    MUSICAL_SYMBOLS,
    ANCIENT_GREEK_MUSICAL_NOTATION,
    KAKTOVIK_NUMERALS,
    MAYAN_NUMERALS,
    TAI_XUAN_JING_SYMBOLS,
    COUNTING_ROD_NUMERALS,
    MATHEMATICAL_ALPHANUMERIC_SYMBOLS,
    SUTTON_SIGNWRITING,
    LATIN_EXTENDED_G,
    GLAGOLITIC_SUPPLEMENT,
    CYRILLIC_EXTENDED_D,
    NYIAKENG_PUACHUE_HMONG,
    TOTO,
    WANCHO,
    NAG_MUNDARI,
    ETHIOPIC_EXTENDED_B,
    MENDE_KIKAKUI,
    ADLAM,
    INDIC_SIYAQ_NUMBERS,
    OTTOMAN_SIYAQ_NUMBERS,
    ARABIC_MATHEMATICAL_ALPHABETIC_SYMBOLS,
    MAHJONG_TILES,
    DOMINO_TILES,
    PLAYING_CARDS,
    ENCLOSED_ALPHANUMERIC_SUPPLEMENT,
    ENCLOSED_IDEOGRAPHIC_SUPPLEMENT,
    MISCELLANEOUS_SYMBOLS_AND_PICTOGRAPHS,
    EMOTICONS,
    ORNAMENTAL_DINGBATS,
    TRANSPORT_AND_MAP_SYMBOLS,
    ALCHEMICAL_SYMBOLS,
    GEOMETRIC_SHAPES_EXTENDED,
    SUPPLEMENTAL_ARROWS_C,
    SUPPLEMENTAL_SYMBOLS_AND_PICTOGRAPHS,
    CHESS_SYMBOLS,
    SYMBOLS_AND_PICTOGRAPHS_EXTENDED_A,
    SYMBOLS_FOR_LEGACY_COMPUTING,
    CJK_UNIFIED_IDEOGRAPHS_EXTENSION_B,
    CJK_UNIFIED_IDEOGRAPHS_EXTENSION_C,
    CJK_UNIFIED_IDEOGRAPHS_EXTENSION_D,
    CJK_UNIFIED_IDEOGRAPHS_EXTENSION_E,
    CJK_UNIFIED_IDEOGRAPHS_EXTENSION_F,
    CJK_UNIFIED_IDEOGRAPHS_EXTENSION_I,
    CJK_COMPATIBILITY_IDEOGRAPHS_SUPPLEMENT,
    CJK_UNIFIED_IDEOGRAPHS_EXTENSION_G,
    CJK_UNIFIED_IDEOGRAPHS_EXTENSION_H,
    TAGS,
    VARIATION_SELECTORS_SUPPLEMENT,
    SUPPLEMENTARY_PRIVATE_USE_AREA_A,
    SUPPLEMENTARY_PRIVATE_USE_AREA_B,
];
