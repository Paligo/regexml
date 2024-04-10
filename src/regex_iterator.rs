use ahash::{HashMap, HashMapExt};

use crate::re_matcher::ReMatcher;

struct Action {}

impl Action {
    fn characters(&mut self, current: &[char]) {
        // nothing yet
    }

    fn on_group_start(&mut self, group: i64) {
        // nothing yet
    }

    fn on_group_end(&mut self, group: i64) {
        // nothing yet
    }
}

struct RegexIterator<'a> {
    // the input string being matched
    the_string: &'a [char],
    regex: &'a [char],
    // the Matcher object that does the matching, and holds the state
    matcher: ReMatcher<'a>,
    // the string most recently returned by the iterator
    current: Option<&'a [char]>,
    // if the last string was a matching string, None, null; otherwise the next
    // substring matched by the regex
    next_substring: Option<&'a [char]>,
    // the position in the input string of the end of the last match or
    // non-match
    prev_end: Option<usize>,
    // evaluated on demand: a table that indicates for each captured group,
    // what its immediately-containing captured group is.
    nesting_table: Option<HashMap<usize, usize>>,
    // indicates the last match was zero length
    skip: bool,
}

impl<'a> RegexIterator<'a> {
    pub(crate) fn new(str: &'a [char], regex: &'a [char], matcher: ReMatcher<'a>) -> Self {
        Self {
            the_string: str,
            regex,
            matcher,
            current: None,
            next_substring: None,
            prev_end: Some(0),
            nesting_table: Some(HashMap::new()),
            skip: false,
        }
    }

    fn is_matching(&self) -> bool {
        self.next_substring.is_none() && self.prev_end.is_some()
    }

    fn get_regex_group(&'a self, number: Option<usize>) -> Option<&'a [char]> {
        if !self.is_matching() {
            return None;
        }
        if let Some(number) = number {
            if number >= self.matcher.paren_count() {
                None
            } else {
                self.matcher.get_paren(number)
            }
        } else {
            None
        }
    }

    fn get_number_of_groups(&self) -> usize {
        self.matcher.paren_count()
    }

    fn process_matching_substring(&mut self, action: &mut Action) {
        let c = self.matcher.paren_count() - 1;
        if c == 0 {
            action.characters(self.current.unwrap());
        } else {
            // Create a map from positions in the string to lists of actions.
            // The "actions" in each list are: +N: start group N; -N: end group
            // N
            let mut actions: HashMap<usize, Vec<i64>> = HashMap::new();
            for i in 1..=c {
                let minus_1 = self.matcher.get_paren_start(0) > self.matcher.get_paren_start(i);
                let start = self.matcher.get_paren_start(i).unwrap()
                    - self.matcher.get_paren_start(0).unwrap();
                if !minus_1 {
                    let end = self.matcher.get_paren_end(i).unwrap()
                        - self.matcher.get_paren_start(0).unwrap();
                    if start < end {
                        // Add the start action after all other actions on the
                        // list for the same position
                        // TODO: converting usize to i64 is not great
                        let s = actions.entry(start).or_insert(Vec::with_capacity(4));
                        s.push(i as i64);

                        let e = actions.entry(end).or_insert(Vec::with_capacity(4));
                        e.insert(0, -(i as i64));
                    }
                } else {
                    // zero-length group (start==end). The problem here is that
                    // the information available from Java isn't sufficient to
                    // determine the nesting of groups: match("a", "(a(b?))")
                    // and match("a", "(a)(b?)") will both give the same result
                    // for group 2 (start=1, end=1). So we need to go back to
                    // the original regex to determine the group nesting
                    let nesting_table = if let Some(nesting_table) = &self.nesting_table {
                        nesting_table
                    } else {
                        self.nesting_table = Some(self.compute_nesting_table(self.regex));
                        self.nesting_table.as_mut().unwrap()
                    };
                    let parent_group = nesting_table.get(&i);
                    // insert the start and end events immediately before the
                    // end event for the parent group, if present; otherwise
                    // after all existing events for this position
                    let s = actions.get_mut(&start);
                    if let Some(s) = s {
                        let mut pos = s.len();
                        for e in 0..s.len() {
                            // TODO usize versus i64
                            if s.get(e) == parent_group.map(|g| -(*g as i64)).as_ref() {
                                pos = e;
                                break;
                            }
                        }
                        s.insert(pos, -(i as i64));
                        s.insert(pos, i as i64);
                    } else {
                        let mut s = Vec::with_capacity(4);
                        s.push(-(i as i64));
                        s.push(i as i64);
                        actions.insert(start, s);
                    }
                }
            }
            let mut buff = Vec::new();

            if let Some(current) = self.current {
                for i in 0..=current.len() {
                    let events = actions.get(&i);
                    if let Some(events) = events {
                        if !buff.is_empty() {
                            action.characters(&buff);
                            buff.clear();
                        }
                        for group in events {
                            if *group > 0 {
                                action.on_group_start(*group);
                            } else {
                                action.on_group_end(-group);
                            }
                        }
                    }

                    if i < current.len() {
                        buff.push(current[i]);
                    }
                }
            }
            if !buff.is_empty() {
                action.characters(&buff);
            }
        }
    }

    fn compute_nesting_table(&self, regex: &[char]) -> HashMap<usize, usize> {
        let mut nesting_table = HashMap::with_capacity(16);
        // TODO: determine capacity based on regex length in unicode chars?
        let mut stack = Vec::new();
        let mut capture_stack = Vec::new();
        let group = 1;
        let mut in_brackets = 0;
        stack.push(0);
        let mut chars = regex.iter().peekable();
        while let Some(ch) = chars.next() {
            if *ch == '\\' {
                continue;
            } else if *ch == '[' {
                in_brackets += 1;
            } else if *ch == ']' {
                in_brackets -= 1;
            } else if *ch == '(' && in_brackets == 0 {
                // peek ahead one
                let capture = chars.peek() != Some(&&'?');
                capture_stack.push(capture);
                if capture {
                    nesting_table.insert(group, stack[stack.len() - 1]);
                    stack.push(group);
                }
            } else if *ch == ')' && in_brackets == 0 {
                let capture = capture_stack.pop().unwrap();
                if capture {
                    stack.pop();
                }
            }
        }
        nesting_table
    }
}

impl<'a> Iterator for RegexIterator<'a> {
    type Item = &'a [char];

    fn next(&mut self) -> Option<Self::Item> {
        if self.next_substring.is_none() && self.prev_end.is_some() {
            // we've returned a match (or we're at the start), so find the next match
            let mut search_start = self.prev_end.unwrap();
            if self.skip {
                // previous match was zero-length
                search_start += 1;
                if search_start >= self.the_string.len() {
                    if self.prev_end.unwrap() < self.the_string.len() {
                        self.current = Some(&self.the_string[self.prev_end.unwrap()..]);
                        return self.current;
                    } else {
                        self.prev_end = None;
                        self.current = None;
                        return self.current;
                    }
                }
            }
            if self.matcher.matches(self.the_string, search_start) {
                let start = self.matcher.get_paren_start(0).unwrap();
                let end = self.matcher.get_paren_end(0).unwrap();
                self.skip = start == end;
                if self.prev_end == Some(start) {
                    // there's no intervening non-matching string to return
                    self.next_substring = None;
                    self.current = Some(&self.the_string[start..end]);
                    self.prev_end = Some(end);
                    self.current
                } else {
                    // return the non-matching substring first
                    self.current = Some(&self.the_string[self.prev_end.unwrap()..start]);
                    self.next_substring = Some(&self.the_string[start..end]);
                    self.current
                }
            } else {
                // there are no more regex matches, we must return the final
                // non-matching text if any
                if self.prev_end.unwrap() < self.the_string.len() {
                    self.current = Some(&self.the_string[self.prev_end.unwrap()..]);
                    self.next_substring = None;
                    self.prev_end = None;
                    self.current
                } else {
                    // this realy is the end...
                    self.prev_end = None;
                    self.current = None;
                    self.current
                }
            }
        } else {
            // we've returned a non-match, so now return the match that follows
            // it, if there is one
            if self.prev_end.is_some() {
                self.current = self.next_substring;
                self.next_substring = None;
                self.prev_end = Some(self.matcher.get_paren_end(0).unwrap());
                self.current
            } else {
                self.current = None;
                self.current
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
