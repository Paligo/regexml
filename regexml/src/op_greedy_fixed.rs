use crate::{
    op_nothing::Nothing,
    operation::{Operation, OperationControl, RepeatOperation, MATCHES_ZLS_ANYWHERE},
    re_flags::ReFlags,
    re_matcher::ReMatcher,
};

// Handle a greedy repetition (with possible min and max) where the size of the
// repeated unit is fixed.
#[derive(Debug, Clone)]
pub(crate) struct GreedyFixed {
    operation: Box<Operation>,
    pub(crate) min: usize,
    max: usize,
    len: usize,
}

impl GreedyFixed {
    pub(crate) fn new(operation: Operation, min: usize, max: usize, len: usize) -> Self {
        Self {
            operation: operation.into(),
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

    fn optimize(&self, flags: &ReFlags) -> Operation {
        if self.max == 0 {
            return Operation::from(Nothing);
        }
        if self.operation.get_match_length() == Some(0) {
            return self.operation.as_ref().clone();
        }
        let operation = self.operation.optimize(flags);
        Operation::from(GreedyFixed {
            operation: operation.into(),
            min: self.min,
            max: self.max,
            len: self.len,
        })
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
            let matched = it.next().is_some();
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

    fn children(&self) -> Vec<Operation> {
        vec![self.operation.as_ref().clone()]
    }
}

impl RepeatOperation for GreedyFixed {
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
        true
    }
}

#[derive(Debug)]
struct IntStepIterator {
    current: i64,
    step: i64,
    limit: i64,
}

impl IntStepIterator {
    pub(crate) fn new(current: usize, step: i64, limit: usize) -> Self {
        Self {
            current: current as i64,
            step,
            limit: limit as i64,
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
        self.current += self.step;
        Some(n as usize)
    }
}
