mod character_class;
mod op_atom;
mod op_back_reference;
mod op_bol;
mod op_capture;
mod op_character_class;
mod op_choice;
mod op_end_program;
mod op_eol;
mod op_greedy_fixed;
mod op_nothing;
mod op_reluctant_fixed;
mod op_repeat;
mod op_sequence;
mod op_unambiguous_repeat;
mod operation;
mod re_compiler;
mod re_flags;
mod re_matcher;
mod re_program;
mod regex_iterator;
mod regular_expression;

use crate::re_compiler::{Error, ReCompiler};
use crate::re_flags::ReFlags;
use crate::re_matcher::ReMatcher;
use crate::re_program::ReProgram;

pub use crate::re_flags::Language;

/// A XML-style regular expression.
pub struct Regex {
    re_program: ReProgram,
}

impl Regex {
    fn new(re: &str, flags: &str, language: Language) -> Result<Self, Error> {
        let re_flags = ReFlags::new(flags, language)?;
        let mut re_compiler = ReCompiler::new(re_flags);
        let pattern = re.chars().collect();
        let re_program = re_compiler.compile(pattern)?;
        Ok(Self { re_program })
    }

    /// Create a regular expression from a string, using XPath 3.1 rules.
    pub fn xpath(re: &str, flags: &str) -> Result<Self, Error> {
        Self::new(re, flags, Language::XPath)
    }

    /// Create a regular expression from a string, using XML Schema 1.1 rules.
    pub fn xsd(re: &str, flags: &str) -> Result<Self, Error> {
        Self::new(re, flags, Language::XSD)
    }

    /// Returns `true` if the argument matches this regular expression.
    pub fn is_match(&self, haystack: &str) -> bool {
        let mut matcher = ReMatcher::new(&self.re_program);
        let search: Vec<char> = haystack.chars().collect();
        matcher.is_match(&search)
    }

    /// Returns a string with all pieces matching this regular expression replaced
    /// by the replacement.
    pub fn replace_all(&self, haystack: &str, replacement: &str) -> Result<String, Error> {
        let mut matcher = ReMatcher::new(&self.re_program);
        let search: Vec<char> = haystack.chars().collect();
        let replacement: Vec<char> = replacement.chars().collect();
        matcher
            .replace(&search, &replacement)
            .map(|chars| chars.into_iter().collect())
    }

    /// Returns a vector of the regular expression tokenized.
    pub fn tokenize(&self, haystack: &str) -> Result<Vec<String>, Error> {
        let mut matcher = ReMatcher::new(&self.re_program);

        let mut result: Vec<String> = Vec::new();
        let mut prev_end = Some(0);
        let input = &haystack.chars().collect::<Vec<_>>();

        while let Some(end) = prev_end {
            let matches = matcher.matches(input, end);
            if matches {
                let start = matcher.get_paren_start(0).unwrap();
                prev_end = matcher.get_paren_end(0);
                result.push(input[end..start].iter().collect())
            } else {
                result.push(input[end..].iter().collect());
                break;
            }
        }
        Ok(result)
    }
}
