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

use ahash::{HashMap, HashMapExt};

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

    /// Returns a vector of the input string tokenized by the regular expression.
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

    /// Use this regular expression to analyze an input string, The resulting
    /// vector provides both the matching and non-matching substrings. It also
    /// provides access to matched subgroups.
    pub fn analyze(&self, haystack: &str) -> Result<Vec<AnalyzeEntry>, Error> {
        todo!();
    }

    // TODO: continue translating ARegexIterator
    // this also has an isMatching protocol, and a processMatchingSubstring story
    // and a computeNestingTable story too. Need to read more into how this is
    // actually used. - it seems vastly complicated.
}

struct AnalyzeIter<'a> {
    // the input string being matched
    the_string: &'a [char],
    regex: &'a [char],
    matcher: ReMatcher<'a>,
    next_substring: Option<&'a [char]>,
    prevend: Option<usize>,
    nesting_table: Option<HashMap<usize, usize>>,
    skip: bool,
}

impl<'a> AnalyzeIter<'a> {
    fn new(the_string: &'a [char], regex: &'a [char], matcher: ReMatcher<'a>) -> Self {
        AnalyzeIter {
            the_string,
            regex,
            matcher,
            // current: None,
            next_substring: None,
            prevend: Some(0),
            nesting_table: None,
            skip: false,
        }
    }

    fn is_matching(&self) -> bool {
        self.next_substring.is_none() && self.prevend.is_some()
    }

    fn process_matching_substring(
        &mut self,
        current: &'a [char],
    ) -> Result<Vec<MatchEntry>, Error> {
        let c = self.matcher.get_paren_count() - 1;
        if c == 0 {
            Ok(vec![MatchEntry::String(current.iter().collect())])
        } else {
            // create a map from positions in the string to lists of actions
            // the "actions" in each list are: +N: start group N, -N: end group N.
            let mut actions: HashMap<usize, Vec<isize>> = HashMap::new();
            for i in 1..=c {
                if let (Some(start_i), Some(start_0)) = (
                    self.matcher.get_paren_start(i),
                    self.matcher.get_paren_start(0),
                ) {
                    let start = start_i - start_0;
                    let end = self.matcher.get_paren_end(i).unwrap() - start_0;
                    if start < end {
                        // add the start action after all other actions on the
                        // list for the same position
                        let s = actions
                            .entry(start)
                            .or_insert_with(|| Vec::with_capacity(4));
                        s.push(i.try_into().unwrap());
                        // add the end action after all other actions on the
                        // list for the same position
                        let e = actions.entry(end).or_insert_with(|| Vec::with_capacity(4));
                        let i: isize = i.try_into().unwrap();
                        e.insert(0, -i);
                    } else {
                        // let nesting_table = if let Some(nesting_table) = &self.nesting_table {
                        //     nesting_table
                        // } else {
                        //     let nesting_table = self.compute_nesting_table(self.regex);
                        //     self.nesting_table = Some(nesting_table);
                        //     &nesting_table
                        // };
                    }
                }
            }
            todo!();
        }
    }

    fn compute_nesting_table(&self, regex: &'a [char]) -> HashMap<usize, usize> {
        let mut nesting_table = HashMap::new();
        let mut stack = Vec::with_capacity(regex.len());
        let mut tos = 0;
        let mut capture_stack = Vec::with_capacity(regex.len());
        let mut capture_tos = 0;
        let mut group = 1;
        let mut in_brackets = 0;
        stack[0] = 0;
        tos += 1;
        let mut i = 0;
        while i < regex.len() {
            let ch = regex[i];
            match ch {
                '\\' => {
                    i += 1;
                }
                '[' => {
                    in_brackets += 1;
                }
                ']' => {
                    in_brackets -= 1;
                }
                '(' if in_brackets == 0 => {
                    let capture = regex[i + 1] != '?';
                    capture_stack[capture_tos] = capture;
                    capture_tos += 1;
                    if capture {
                        nesting_table.insert(group, stack[tos - 1]);
                        stack[tos] = group;
                        tos += 1;
                        group += 1;
                    }
                }
                ')' if in_brackets == 0 => {
                    capture_tos -= 1;
                    let capture = capture_stack[capture_tos];
                    if capture {
                        tos -= 1;
                    }
                }
                _ => {}
            }
            i += 1;
        }
        nesting_table
    }
}

impl<'a> Iterator for AnalyzeIter<'a> {
    type Item = &'a [char];

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(prevend) = self.prevend {
            if let Some(substring) = self.next_substring {
                // we've added a non-match, so now added the match that follows
                // it, if there is one
                let current = substring;
                self.next_substring = None;
                self.prevend = self.matcher.get_paren_end(0);
                Some(current)
            } else {
                // we've returned a match (or we're at the start) so find the
                // next match
                let mut search_start = prevend;
                if self.skip {
                    // previous match was zero-length
                    search_start += 1;
                    if search_start >= self.the_string.len() {
                        if prevend < self.the_string.len() {
                            self.next_substring = None;
                        } else {
                            self.prevend = None;
                            return None;
                        }
                    }
                }
                if self.matcher.matches(self.the_string, search_start) {
                    let start = self.matcher.get_paren_start(0).unwrap();
                    let end = self.matcher.get_paren_end(0).unwrap();
                    self.skip = start == end;
                    if prevend == start {
                        // there's no intervening non-matching string to return
                        self.next_substring = None;
                        self.prevend = Some(end);
                        Some(&self.the_string[start..end])
                    } else {
                        // return the non-matching substring first
                        self.next_substring = Some(&self.the_string[start..end]);
                        Some(&self.the_string[prevend..start])
                    }
                } else {
                    // there are no more regex matches, we must return the final non-match
                    if prevend < self.the_string.len() {
                        self.next_substring = None;
                        Some(&self.the_string[prevend..])
                    } else {
                        // this really is the end...
                        self.prevend = None;
                        None
                    }
                }
            }
        } else {
            None
        }
    }
}

pub enum AnalyzeEntry {
    Match(Vec<MatchEntry>),
    NonMatch(String),
}

pub enum MatchEntry {
    String(String),
    Group { nr: usize, value: String },
}
