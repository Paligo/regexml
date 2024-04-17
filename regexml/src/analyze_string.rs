use crate::re_matcher::ReMatcher;
use ahash::{HashMap, HashMapExt};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum AnalyzeEntry {
    Match(Vec<MatchEntry>),
    NonMatch(String),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum MatchEntry {
    String(String),
    Group { nr: usize, value: String },
}

pub struct AnalyzeIter<'a> {
    matcher: ReMatcher<'a>,
    next_substring: Option<Vec<char>>,
    prev_end: Option<usize>,
    nesting_table: HashMap<usize, usize>,
    skip: bool,
}

impl<'a> AnalyzeIter<'a> {
    pub(crate) fn new(pattern: &'a [char], matcher: ReMatcher<'a>) -> Self {
        AnalyzeIter {
            matcher,
            next_substring: None,
            prev_end: Some(0),
            nesting_table: Self::compute_nesting_table(pattern),
            skip: false,
        }
    }

    fn analyze_entry(&self, current: &[char]) -> AnalyzeEntry {
        if self.is_matching() {
            AnalyzeEntry::Match(self.process_matching_substring(current))
        } else {
            AnalyzeEntry::NonMatch(current.iter().collect::<String>())
        }
    }

    fn is_matching(&self) -> bool {
        self.next_substring.is_none() && self.prev_end.is_some()
    }

    fn process_matching_substring(&self, current: &[char]) -> Vec<MatchEntry> {
        let c = self.matcher.paren_count() - 1;
        if c == 0 {
            vec![MatchEntry::String(current.iter().collect())]
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
            result
        }
    }

    fn compute_nesting_table(pattern: &'a [char]) -> HashMap<usize, usize> {
        let mut nesting_table = HashMap::new();
        let mut stack = vec![0; pattern.len()];
        let mut tos = 0;
        let mut capture_stack = vec![false; pattern.len()];
        let mut capture_tos = 0;
        let mut group = 1;
        let mut in_brackets = 0;
        tos += 1;
        let mut i = 0;
        while i < pattern.len() {
            let ch = pattern[i];
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
                    let capture = pattern[i + 1] != '?';
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
    type Item = AnalyzeEntry;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(prev_end) = self.prev_end {
            if let Some(substring) = self.next_substring.take() {
                // we've added a non-match, so now added the match that follows
                // it, if there is one
                if self.prev_end.is_some() {
                    self.prev_end = self.matcher.get_paren_end(0);
                    Some(self.analyze_entry(&substring))
                } else {
                    None
                }
            } else {
                // we've returned a match (or we're at the start) so find the
                // next match
                let mut search_start = prev_end;
                if self.skip {
                    // previous match was zero-length
                    search_start += 1;
                    if search_start >= self.matcher.search.len() {
                        if prev_end < self.matcher.search.len() {
                            self.next_substring = None;
                        } else {
                            self.prev_end = None;
                            return None;
                        }
                    }
                }

                if self.matcher.matches(search_start) {
                    let start = self.matcher.get_paren_start(0).unwrap();
                    let end = self.matcher.get_paren_end(0).unwrap();
                    self.skip = start == end;
                    if prev_end == start {
                        // there's no intervening non-matching string to return
                        self.next_substring = None;
                        self.prev_end = Some(end);
                        Some(self.analyze_entry(&self.matcher.search[start..end]))
                    } else {
                        // return the non-matching substring first
                        self.next_substring = Some(self.matcher.search[start..end].to_vec());
                        Some(self.analyze_entry(&self.matcher.search[prev_end..start]))
                    }
                } else {
                    // there are no more regex matches, we must return the final non-match
                    if prev_end < self.matcher.search.len() {
                        self.next_substring = None;
                        let non_match = AnalyzeEntry::NonMatch(
                            self.matcher.search[prev_end..].iter().collect(),
                        );
                        self.prev_end = None;
                        Some(non_match)
                    } else {
                        // this really is the end...
                        self.prev_end = None;
                        None
                    }
                }
            }
        } else {
            None
        }
    }
}
