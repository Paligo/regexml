#[cfg(test)]
use crate::operation::Operation;
use crate::re_compiler::ReCompiler;
use crate::re_flags::ReFlags;
use crate::re_matcher::ReMatcher;
use crate::re_program::ReProgram;
#[cfg(test)]
use std::rc::Rc;

pub use crate::analyze_string::AnalyzeIter;
pub use crate::re_compiler::Error;
pub use crate::re_flags::Language;

/// A XML-style regular expression.
#[derive(Debug)]
pub struct Regex {
    re_program: ReProgram,
    matches_empty_string: bool,
}

impl Regex {
    fn new(re: &str, flags: &str, language: Language) -> Result<Self, Error> {
        let re_flags = ReFlags::new(flags, language)?;
        let pattern = re.chars().collect();
        let re_compiler = ReCompiler::new(pattern, re_flags);
        let re_program = re_compiler.compile()?;
        // we need to check if the regex matches the empty string
        let mut matcher = ReMatcher::new(&re_program, "");
        let matches_empty_string = matcher.is_match();
        Ok(Self {
            re_program,
            matches_empty_string,
        })
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
        let mut matcher = self.matcher(haystack);
        matcher.is_match()
    }

    // returns an error if this regex is known to match an empty string.
    // caches the last result so it doesn't have to do the match again.
    fn check_matches_empty_string(&self) -> Result<(), Error> {
        if !self.matches_empty_string {
            Ok(())
        } else {
            Err(Error::MatchesEmptyString)
        }
    }

    /// Returns a string with all pieces matching this regular expression replaced
    /// by the replacement.
    pub fn replace_all(&self, haystack: &str, replacement: &str) -> Result<String, Error> {
        self.check_matches_empty_string()?;

        let mut matcher = self.matcher(haystack);
        let replacement: Vec<char> = replacement.chars().collect();
        matcher
            .replace(&replacement)
            .map(|chars| chars.into_iter().collect())
    }

    /// Returns an iterator of the input string tokenized by the regular expression.
    pub fn tokenize<'a>(&'a self, haystack: &str) -> Result<TokenIter<'a>, Error> {
        // if we input the empty string, we should return no tokens
        if haystack.is_empty() {
            return Ok(TokenIter {
                matcher: self.matcher(haystack),
                prev_end: None,
            });
        }
        self.check_matches_empty_string()?;

        Ok(TokenIter {
            matcher: self.matcher(haystack),
            prev_end: Some(0),
        })
    }

    pub(crate) fn matcher(&self, search: &str) -> ReMatcher {
        ReMatcher::new(&self.re_program, search)
    }

    #[cfg(test)]
    pub(crate) fn path(&self, s: &str) -> Rc<Operation> {
        self.re_program.path(s)
    }

    /// Use this regular expression to analyze an input string, The resulting
    /// vector provides both the matching and non-matching substrings. It also
    /// provides access to matched subgroups.
    pub fn analyze<'a>(&'a self, haystack: &str) -> Result<AnalyzeIter<'a>, Error> {
        self.check_matches_empty_string()?;
        Ok(AnalyzeIter::new(
            &self.re_program.pattern,
            self.matcher(haystack),
        ))
    }

    // TODO: continue translating ARegexIterator
    // this also has an isMatching protocol, and a processMatchingSubstring story
    // and a computeNestingTable story too. Need to read more into how this is
    // actually used. - it seems vastly complicated.
}

#[derive(Debug)]
pub struct TokenIter<'a> {
    matcher: ReMatcher<'a>,
    prev_end: Option<usize>,
}

impl<'a> Iterator for TokenIter<'a> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(prev_end) = self.prev_end {
            if self.matcher.matches(prev_end) {
                let start = self.matcher.get_paren_start(0).unwrap();
                let current = self.matcher.search[prev_end..start].iter().collect();
                self.prev_end = self.matcher.get_paren_end(0);
                Some(current)
            } else {
                let current = self.matcher.search[prev_end..].iter().collect();
                self.prev_end = None;
                Some(current)
            }
        } else {
            None
        }
    }
}
