mod block;
mod category;
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

use std::cell::{Ref, RefCell};
use std::rc::Rc;

use ahash::{HashMap, HashMapExt};
use operation::Operation;

use crate::re_compiler::ReCompiler;
use crate::re_flags::ReFlags;
use crate::re_matcher::ReMatcher;
use crate::re_program::ReProgram;

pub use crate::re_flags::Language;

pub use crate::re_compiler::Error;

/// A XML-style regular expression.
#[derive(Debug)]
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
        let mut matcher = self.matcher(haystack);
        matcher.is_match()
    }

    /// Returns a string with all pieces matching this regular expression replaced
    /// by the replacement.
    pub fn replace_all(&self, haystack: &str, replacement: &str) -> Result<String, Error> {
        let mut matcher = self.matcher(haystack);
        let replacement: Vec<char> = replacement.chars().collect();
        matcher
            .replace(&replacement)
            .map(|chars| chars.into_iter().collect())
    }

    /// Returns a vector of the input string tokenized by the regular expression.
    pub fn tokenize(&self, haystack: &str) -> Result<Vec<String>, Error> {
        let mut matcher = self.matcher(haystack);

        let mut result: Vec<String> = Vec::new();
        let mut prev_end = Some(0);

        while let Some(end) = prev_end {
            let matches = matcher.matches(end);
            if matches {
                let start = matcher.get_paren_start(0).unwrap();
                prev_end = matcher.get_paren_end(0);
                result.push(matcher.search[end..start].iter().collect())
            } else {
                result.push(matcher.search[end..].iter().collect());
                break;
            }
        }
        Ok(result)
    }

    pub(crate) fn matcher(&self, search: &str) -> ReMatcher {
        ReMatcher::new(&self.re_program, search)
    }

    pub(crate) fn path(&self, s: &str) -> Rc<Operation> {
        self.re_program.path(s)
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
    matcher: ReMatcher<'a>,
    next_substring: Option<&'a [char]>,
    prevend: Option<usize>,
    nesting_table: HashMap<usize, usize>,
    skip: bool,
}

impl<'a> AnalyzeIter<'a> {
    fn new(the_string: &'a [char], regex: &'a [char], matcher: ReMatcher<'a>) -> Self {
        AnalyzeIter {
            the_string,
            matcher,
            // current: None,
            next_substring: None,
            prevend: Some(0),
            nesting_table: Self::compute_nesting_table(regex),
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
        let c = self.matcher.paren_count() - 1;
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
                        let parent_group = self.nesting_table.get(&i).unwrap();
                        // insert the start and end events immediately before
                        // the end event for the parent group, if present;
                        // otherwise after all existing events for this
                        // position
                        actions
                            .entry(start)
                            .and_modify(|v| {
                                let mut pos = v.len();
                                for e in 0..v.len() {
                                    let parent_group: isize = (*parent_group).try_into().unwrap();
                                    if v.get(e) == Some(&-parent_group) {
                                        pos = e;
                                        break;
                                    }
                                }
                                let i: isize = i.try_into().unwrap();
                                v.insert(pos, -i);
                                v.insert(pos, i);
                            })
                            .or_insert_with(|| {
                                let mut v = Vec::with_capacity(4);
                                let i: isize = i.try_into().unwrap();
                                v.push(-i);
                                v.push(i);
                                v
                            });
                    }
                }
            }

            let mut result = Vec::new();
            let mut buff = Some(String::new());
            let mut in_group = None;
            for i in 0..=current.len() {
                let events = actions.get(&i);
                if let Some(events) = events {
                    if let Some(buff) = buff.take() {
                        let match_entry = if let Some(in_group) = in_group {
                            MatchEntry::Group {
                                nr: in_group,
                                value: buff,
                            }
                        } else {
                            MatchEntry::String(buff)
                        };
                        result.push(match_entry);
                    }
                    for group in events {
                        if group > &0 {
                            in_group = Some((*group).try_into().unwrap());
                        } else {
                            in_group = None;
                        }
                    }
                }
                if i < current.len() {
                    if let Some(buff) = &mut buff {
                        buff.push(current[i]);
                    } else {
                        let mut s = String::new();
                        s.push(current[i]);
                        buff = Some(s)
                    }
                }
            }
            Ok(result)
        }
    }

    fn compute_nesting_table(regex: &'a [char]) -> HashMap<usize, usize> {
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
                if self.matcher.matches(search_start) {
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
