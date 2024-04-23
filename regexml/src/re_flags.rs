use crate::re_compiler::Error;

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum Language {
    XSD,
    XPath,
}

#[derive(Debug, Clone)]
pub(crate) struct ReFlags {
    case_independent: bool,
    multi_line: bool,
    single_line: bool,
    allow_whitespace: bool,
    literal: bool,
    language: Language,
    debug: bool,                     // flags = ';g'
    allow_unknown_block_names: bool, // flags = ';k'
}

impl ReFlags {
    pub(crate) fn new(flags: &str, language: Language) -> Result<Self, Error> {
        let mut r = Self {
            case_independent: false,
            multi_line: false,
            single_line: false,
            allow_whitespace: false,
            literal: false,
            language,
            debug: false,
            allow_unknown_block_names: false,
        };

        let mut chars = flags.chars();

        for c in chars.by_ref() {
            if c == ';' {
                break;
            }
            match c {
                'i' => {
                    r.case_independent = true;
                }
                'm' => {
                    r.multi_line = true;
                }
                's' => {
                    r.single_line = true;
                }
                'q' => {
                    r.literal = true;
                    if language != Language::XPath {
                        return Err(Error::InvalidFlags(
                            "'q' flag requires XPath 3.0 to be enabled".to_string(),
                        ));
                    }
                }
                'x' => {
                    r.allow_whitespace = true;
                }
                _ => {
                    return Err(Error::InvalidFlags(format!("Unrecognized flag '{}'", c)));
                }
            }
        }
        // after ';'
        for c in chars {
            match c {
                'g' => {
                    r.debug = true;
                }
                'k' => {
                    r.allow_unknown_block_names = true;
                }
                'K' => {
                    r.allow_unknown_block_names = false;
                }
                _ => {
                    return Err(Error::InvalidFlags(format!("Unrecognized flag '{}'", c)));
                }
            }
        }
        Ok(r)
    }

    pub(crate) fn is_case_independent(&self) -> bool {
        self.case_independent
    }

    pub(crate) fn is_multi_line(&self) -> bool {
        self.multi_line
    }

    pub(crate) fn is_single_line(&self) -> bool {
        self.single_line
    }

    pub(crate) fn is_allow_whitespace(&self) -> bool {
        self.allow_whitespace
    }

    pub(crate) fn is_literal(&self) -> bool {
        self.literal
    }

    pub(crate) fn language(&self) -> Language {
        self.language
    }

    // This appears to do nothing in the Java original; it's triggered
    // by the 'g' flag
    // pub(crate) fn is_debug(&self) -> bool {
    //     self.debug
    // }

    // We haven't implemented this support yet; in the Java original it
    // gives a warning.
    // pub(crate) fn is_allow_unknown_block_names(&self) -> bool {
    //     self.allow_unknown_block_names
    // }
}
