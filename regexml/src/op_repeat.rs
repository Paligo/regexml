use crate::{
    character_class::CharacterClass,
    operation::{
        ForceProgressIterator, Operation, OperationControl, RepeatOperation, MATCHES_ZLS_ANYWHERE,
    },
    re_flags::ReFlags,
    re_matcher::ReMatcher,
};

// Handle a repetition (with possible min and max) where the
// size of the repeated unit is variable.
#[derive(Debug, Clone)]
pub(crate) struct Repeat {
    pub(crate) operation: Box<Operation>,
    pub(crate) min: usize,
    pub(crate) max: usize,
    pub(crate) greedy: bool,
}

impl Repeat {
    pub(crate) fn new(operation: Operation, min: usize, max: usize, greedy: bool) -> Self {
        Self {
            operation: Box::new(operation),
            min,
            max,
            greedy,
        }
    }
}

impl OperationControl for Repeat {
    fn get_match_length(&self) -> Option<usize> {
        self.operation.get_match_length().and_then(|match_length| {
            if self.min == self.max {
                Some(self.min * match_length)
            } else {
                None
            }
        })
    }

    fn get_minimum_match_length(&self) -> usize {
        self.min * self.operation.get_minimum_match_length()
    }

    fn get_initial_character_class(&self, case_blind: bool) -> CharacterClass {
        self.operation.get_initial_character_class(case_blind)
    }

    fn optimize(self, flags: &ReFlags) -> Operation {
        let operation = self.operation.optimize(flags);
        let min = if self.min == 0 && operation.matches_empty_string() == MATCHES_ZLS_ANYWHERE {
            // turns (a?)* into (a?)+
            1
        } else {
            self.min
        };
        Operation::from(Repeat {
            operation: Box::new(operation),
            min,
            max: self.max,
            greedy: self.greedy,
        })
    }

    fn matches_empty_string(&self) -> u32 {
        if self.min == 0 {
            MATCHES_ZLS_ANYWHERE
        } else {
            self.operation.matches_empty_string()
        }
    }

    fn contains_capturing_expressions(&self) -> bool {
        matches!(self.operation.as_ref(), Operation::Capture(_))
            || self.operation.contains_capturing_expressions()
    }

    fn matches_iter<'a>(
        &'a self,
        matcher: &'a ReMatcher,
        position: usize,
    ) -> Box<dyn Iterator<Item = usize> + 'a> {
        let mut iterators: Vec<Box<dyn Iterator<Item = usize>>> = Vec::new();
        let mut positions = Vec::new();
        let bound = self.max.min(matcher.search.len() - position + 1);
        let mut p = position;
        if self.greedy {
            // Prime the arrays first with iterators up to the maximum length,
            // stopping if there is no match

            // Porting note: is_duplicate_zero_length_match can be dedicated to
            // the Repeat (and not the various subclasses in the original Java
            // code, i.e. GreedyFixed, ReluctantFixed, UnambiguousRepeat),
            // because each of these subclasses overrides matches_iter anyway,
            // so this code can never be reached.
            if self.min == 0 && !matcher.is_duplicate_zero_length_match(self, position) {
                // add a match at the current position if zero occurrences are allowed
                iterators.push(Box::new(std::iter::once(position)));
                positions.push(p);
            }
            for _i in 0..bound {
                let mut it = self.operation.matches_iter(matcher, p);
                if let Some(next) = it.next() {
                    p = next;
                    iterators.push(it);
                    positions.push(p);
                } else if iterators.is_empty() {
                    return Box::new(std::iter::empty());
                } else {
                    break;
                }
            }
            // Now return an iterator which returns all the matching positions
            // in order
            Box::new(ForceProgressIterator::new(Box::new(
                GreedyRepeatIterator::new(
                    matcher,
                    self.operation.as_ref(),
                    iterators,
                    positions,
                    bound,
                    self.min,
                ),
            )))
        } else {
            // reluctant (non-greedy) repeat.
            Box::new(ForceProgressIterator::new(Box::new(
                ReluctantRepeatIterator::new(
                    matcher,
                    self.operation.as_ref(),
                    position,
                    self.min,
                    self.max,
                ),
            )))
        }
    }

    fn children(&self) -> Vec<Operation> {
        vec![self.operation.as_ref().clone()]
    }
}

impl RepeatOperation for Repeat {
    fn child(&self) -> Operation {
        self.operation.as_ref().clone()
    }

    fn min(&self) -> usize {
        self.min
    }

    fn max(&self) -> usize {
        self.max
    }

    fn greedy(&self) -> bool {
        self.greedy
    }
}

struct GreedyRepeatIterator<'a> {
    primed: bool,
    matcher: &'a crate::re_matcher::ReMatcher<'a>,
    operation: &'a Operation,
    min: usize,
    iterators: Vec<Box<dyn Iterator<Item = usize> + 'a>>,
    positions: Vec<usize>,
    bound: usize,
}

impl<'a> GreedyRepeatIterator<'a> {
    fn new(
        matcher: &'a ReMatcher<'a>,
        operation: &'a Operation,
        iterators: Vec<Box<dyn Iterator<Item = usize> + 'a>>,
        positions: Vec<usize>,
        bound: usize,
        min: usize,
    ) -> Self {
        Self {
            primed: true,
            matcher,
            operation,
            min,
            iterators,
            positions,
            bound,
        }
    }
}

impl Iterator for GreedyRepeatIterator<'_> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let has_next = if self.primed && self.iterators.len() >= self.min {
            !self.iterators.is_empty()
        } else if self.iterators.is_empty() {
            false
        } else {
            loop {
                let top = self.iterators.last_mut().unwrap();
                if let Some(mut p) = top.next() {
                    self.positions.pop();
                    self.positions.push(p);
                    while self.iterators.len() < self.bound {
                        let mut it = self.operation.matches_iter(self.matcher, p);
                        if let Some(next) = it.next() {
                            p = next;
                            self.iterators.push(it);
                            self.positions.push(p)
                        } else {
                            break;
                        }
                    }
                } else {
                    self.iterators.pop();
                    self.positions.pop();
                }
                if self.iterators.len() >= self.min || self.iterators.is_empty() {
                    break;
                }
            }
            !self.iterators.is_empty()
        };
        if has_next {
            self.primed = false;
            self.positions.last().copied()
        } else {
            None
        }
    }
}

struct ReluctantRepeatIterator<'a> {
    matcher: &'a crate::re_matcher::ReMatcher<'a>,
    operation: &'a Operation,
    min: usize,
    max: usize,
    counter: usize,
    position: Option<usize>,
}

impl<'a> ReluctantRepeatIterator<'a> {
    fn new(
        matcher: &'a ReMatcher<'a>,
        operation: &'a Operation,
        position: usize,
        min: usize,
        max: usize,
    ) -> Self {
        Self {
            matcher,
            operation,
            min,
            max,
            counter: 0,
            position: Some(position),
        }
    }
}

impl Iterator for ReluctantRepeatIterator<'_> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(position) = self.position {
                let mut it = self.operation.matches_iter(self.matcher, position);
                if let Some(position) = it.next() {
                    self.counter += 1;
                    if self.counter > self.max {
                        self.position = None;
                    } else {
                        self.position = Some(position);
                    }
                }
            } else if self.min == 0 && self.counter == 0 {
                self.counter += 1;
            } else {
                self.position = None;
            }
            if self.counter >= self.min || self.position.is_none() {
                break;
            }
        }
        self.position
    }
}

#[cfg(test)]
mod tests {
    use crate::Regex;

    #[test]
    fn test_repeat_simple() {
        let regex = Regex::xpath(r#"a*"#, "").unwrap();
        let op = regex.path("0");

        // unambiguous repeat
        let matches = regex.matcher("a").operation_matches(op.clone());
        assert_eq!(matches, vec!["a"]);

        let matches = regex.matcher("").operation_matches(op);
        assert_eq!(matches, vec![""]);
    }

    #[test]
    fn test_repeat_choice() {
        let regex = Regex::xpath(r#"(?:a|b)*"#, "").unwrap();
        let op = regex.path("0");

        let matches = regex.matcher("").operation_matches(op.clone());
        assert_eq!(matches, vec![""]);

        let matches = regex.matcher("a").operation_matches(op.clone());
        assert_eq!(matches, vec!["a", ""]);
        let matches = regex.matcher("aba").operation_matches(op.clone());
        assert_eq!(matches, vec!["aba", "ab", "a", ""]);

        let matches = regex.matcher("bab").operation_matches(op);
        assert_eq!(matches, vec!["bab", "ba", "b", ""]);
    }

    #[test]
    fn test_repeat_choice_with_option_first() {
        let regex = Regex::xpath(r#"(?:a?|b)*"#, "").unwrap();
        let op = regex.path("0");

        let matches = regex.matcher("bab").operation_matches(op);

        assert!(matches.contains(&"bab".to_string()));
    }

    #[test]
    fn test_repeat_option() {
        let regex = Regex::xpath(r#"(?:a?)*"#, "").unwrap();
        let op = regex.path("0");

        let matches = regex.matcher("aaa").operation_matches(op);

        assert!(matches.contains(&"aaa".to_string()));
    }

    #[test]
    fn test_repeat_choice_with_option_last() {
        let regex = Regex::xpath(r#"(?:b|a?)*"#, "").unwrap();
        let op = regex.path("0");

        let matches = regex.matcher("bab").operation_matches(op);
        assert_eq!(matches[0], "bab");
    }

    // #[test]
    // fn test_repeat_with_atom_behind() {
    //     let regex = Regex::xpath(r#"^(.*)+B"#, "").unwrap();
    //     let op = regex.path("0");

    //     let matches = regex.matcher("AB").operation_matches(op);
    //     assert_eq!(matches, vec!["AB"]);
    // }
}
