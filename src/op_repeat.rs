use std::rc::Rc;

use crate::{
    operation::{
        ForceProgressIterator, Operation, OperationControl, RepeatOperation, MATCHES_ZLS_ANYWHERE,
    },
    re_matcher::ReMatcher,
};

#[derive(Debug, Clone)]
pub(crate) struct Repeat {
    pub(crate) operation: Rc<Operation>,
    pub(crate) min: usize,
    max: usize,
    greedy: bool,
}

impl Repeat {
    pub(crate) fn new(operation: Rc<Operation>, min: usize, max: usize, greedy: bool) -> Self {
        Self {
            operation,
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
        &self,
        matcher: &'a ReMatcher,
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
                    .state
                    .borrow_mut()
                    .history
                    .is_duplicate_zero_length_match(Operation::from(self.clone()), position)
            {
                // add a match at the current position if zero occurrences are allowed
                iterators.push(Box::new(std::iter::once(position)));
                positions.push(p);
            }
            for _i in 0..bound {
                let mut it = self.operation.matches_iter(matcher, p);
                if let Some(next) = it.next() {
                    p = next;
                    iterators.push(it);
                    positions.push(next);
                } else if iterators.is_empty() {
                    return Box::new(std::iter::empty());
                } else {
                    break;
                }
            }
            // Now return an iterator which returns all the matching positions in order
            Box::new(ForceProgressIterator::new(Box::new(
                GreedyRepeatIterator::new(
                    matcher,
                    self.operation.clone(),
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
                    self.operation.clone(),
                    self.min,
                    self.max,
                    position,
                ),
            )))
        }
    }

    fn display(&self) -> String {
        todo!()
    }
}

impl RepeatOperation for Repeat {
    fn child(&self) -> Rc<Operation> {
        self.operation.clone()
    }

    fn min(&self) -> usize {
        self.min
    }
}

struct GreedyRepeatIterator<'a> {
    primed: bool,
    matcher: &'a crate::re_matcher::ReMatcher<'a>,
    operation: Rc<Operation>,
    min: usize,
    iterators: Vec<Box<dyn Iterator<Item = usize> + 'a>>,
    positions: Vec<usize>,
    bound: usize,
}

impl<'a> GreedyRepeatIterator<'a> {
    fn new(
        matcher: &'a ReMatcher<'a>,
        operation: Rc<Operation>,
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

impl<'a> Iterator for GreedyRepeatIterator<'a> {
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

struct ReluctantRepeatIterator<'a> {
    matcher: &'a crate::re_matcher::ReMatcher<'a>,
    operation: Rc<Operation>,
    min: usize,
    max: usize,
    counter: usize,
    position: Option<usize>,
}

impl<'a> ReluctantRepeatIterator<'a> {
    fn new(
        matcher: &'a ReMatcher<'a>,
        operation: Rc<Operation>,
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

impl<'a> Iterator for ReluctantRepeatIterator<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(position) = self.position {
            loop {
                let mut it = self.operation.matches_iter(self.matcher, position);
                if let Some(position) = it.next() {
                    self.position = Some(position);
                    self.counter += 1;
                    if self.counter > self.max {
                        self.position = None;
                        break;
                    }
                } else if self.min == 0 && self.counter == 0 {
                    self.counter += 1;
                } else {
                    self.position = None;
                    break;
                }
                if self.counter >= self.min {
                    break;
                }
            }
        }
        self.position
    }
}
