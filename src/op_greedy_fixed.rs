use std::rc::Rc;

use crate::{
    operation::{Operation, OperationControl, MATCHES_ZLS_ANYWHERE},
    re_matcher::ReMatcher,
};

#[derive(Clone)]
pub(crate) struct GreedyFixed {
    operation: Rc<Operation>,
    min: usize,
    max: usize,

    len: usize,
}

impl GreedyFixed {
    pub(crate) fn new(operation: Rc<Operation>, min: usize, max: usize, len: usize) -> Self {
        Self {
            operation,
            min,
            max,
            len,
        }
    }
}

impl OperationControl for GreedyFixed {
    fn get_match_length(&self) -> Option<usize> {
        if self.min == self.max {
            Some(self.min * self.len)
        } else {
            None
        }
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

    fn matches_iter<'a>(
        &self,
        matcher: &'a ReMatcher,
        position: usize,
    ) -> Box<dyn Iterator<Item = usize> + 'a> {
        let mut guard = matcher.search.len();
        if self.max < usize::MAX {
            guard = guard.min(position + self.len * self.max)
        }
        if position >= guard && self.min > 0 {
            return Box::new(std::iter::empty());
        }

        let mut p = position;
        let mut matches = 0;
        while p <= guard {
            let mut it = self.operation.matches_iter(matcher, p);
            let mut matched = false;
            if let Some(next) = it.next() {
                matched = true;
            }
            if matched {
                matches += 1;
                p += self.len;
                if matches == self.max {
                    break;
                }
            } else {
                break;
            }
        }
        if matches < self.min {
            return Box::new(std::iter::empty());
        }
        Box::new(IntStepIterator::new(
            p,
            -(self.len as i64),
            position + self.len * self.min,
        ))
    }

    fn display(&self) -> String {
        todo!();
    }
}

struct IntStepIterator {
    current: usize,
    step: i64,
    limit: usize,
}

impl IntStepIterator {
    pub(crate) fn new(current: usize, step: i64, limit: usize) -> Self {
        Self {
            current,
            step,
            limit,
        }
    }
}

impl Iterator for IntStepIterator {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let has_next = if self.step > 0 {
            self.current <= self.limit
        } else {
            self.current >= self.limit
        };
        if !has_next {
            return None;
        }
        let n = self.current;
        if self.step < 0 {
            self.current -= self.step.abs() as usize;
        } else {
            self.current += self.step as usize;
        }
        Some(n)
    }
}
