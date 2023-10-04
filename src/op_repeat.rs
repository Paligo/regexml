use std::rc::Rc;

use crate::{
    operation::{Operation, MATCHES_ZLS_ANYWHERE},
    re_matcher::ReMatcher,
};

struct OpRepeat {
    operation: Rc<dyn Operation>,
    min: usize,
    max: usize,
    greedy: bool,
}

impl OpRepeat {
    fn new(operation: Rc<dyn Operation>, min: usize, max: usize, greedy: bool) -> Self {
        Self {
            operation,
            min,
            max,
            greedy,
        }
    }
}

impl Operation for OpRepeat {
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

    fn matches_empty_string(&self) -> u32 {
        if self.min == 0 {
            MATCHES_ZLS_ANYWHERE
        } else {
            self.operation.matches_empty_string()
        }
    }

    // TODO
    // fn contains_capturing_expressions() -> bool {}

    fn matches_iter<'a>(
        &self,
        matcher: &'a crate::re_matcher::ReMatcher,
        position: usize,
    ) -> Box<dyn Iterator<Item = usize> + 'a> {
        let mut iterators: Vec<Box<dyn Iterator<Item = usize>>> = Vec::new();
        let mut positions = Vec::new();
        let bound = self.max.min(matcher.search.len() - position + 1);
        let mut p = position;
        if self.greedy {
            // Prime the arrays first with iterators up to the maximum length, stopping if there is no match
            if self.min == 0
                && !matcher
                    .history
                    .is_duplicate_zero_length_match(Box::new(self), position)
            {
                // add a match at the current position if zero occurrences are allowed
                iterators.push(Box::new(std::iter::once(position)));
                positions.push(p);
            }
            for _i in 0..bound {
                let mut it = self.operation.matches_iter(matcher, p);
                if let Some(next) = it.next() {
                    iterators.push(it.into());
                    positions.push(next);
                } else if iterators.is_empty() {
                    return Box::new(std::iter::empty());
                } else {
                    break;
                }
            }
            // Now return an iterator which returns all the matching positions in order
            Box::new(RepeatIterator::new(
                matcher,
                self.operation.clone(),
                iterators,
                positions,
                bound,
            ))
        } else {
            todo!()
        }
    }

    fn display(&self) -> String {
        todo!()
    }
}

struct RepeatIterator<'a> {
    primed: bool,
    matcher: &'a crate::re_matcher::ReMatcher<'a>,
    operation: Rc<dyn Operation + 'a>,
    min: usize,
    iterators: Vec<Box<dyn Iterator<Item = usize> + 'a>>,
    positions: Vec<usize>,
    bound: usize,
}

impl<'a> RepeatIterator<'a> {
    fn new(
        matcher: &'a ReMatcher<'a>,
        operation: Rc<dyn Operation + 'a>,
        iterators: Vec<Box<dyn Iterator<Item = usize> + 'a>>,
        positions: Vec<usize>,
        bound: usize,
    ) -> Self {
        Self {
            primed: true,
            matcher,
            operation,
            min: 0,
            iterators,
            positions,
            bound,
        }
    }
}

impl<'a> Iterator for RepeatIterator<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let has_next = if self.primed && self.iterators.len() >= self.min {
            !self.iterators.is_empty()
        } else if self.iterators.is_empty() {
            false
        } else {
            loop {
                let top = self.iterators.last_mut().unwrap();
                let p = top.next();
                if let Some(p) = p {
                    self.positions.pop();
                    self.positions.push(p);
                    while self.iterators.len() < self.bound {
                        let operation = &self.operation;
                        let mut it = operation.matches_iter(self.matcher, p);
                        if let Some(p) = it.next() {
                            self.iterators.push(it.into());
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
            self.iterators.is_empty()
        };
        if has_next {
            self.primed = false;
            self.positions.last().cloned()
        } else {
            None
        }
    }
}
