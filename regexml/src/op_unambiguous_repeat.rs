use std::rc::Rc;

use crate::{
    operation::{Operation, OperationControl, RepeatOperation, MATCHES_ZLS_ANYWHERE},
    re_matcher::ReMatcher,
};

#[derive(Debug, Clone)]
pub(crate) struct UnambiguousRepeat {
    operation: Rc<Operation>,
    pub(crate) min: usize,
    max: usize,
}

impl UnambiguousRepeat {
    pub(crate) fn new(operation: Rc<Operation>, min: usize, max: usize) -> Self {
        Self {
            operation,
            min,
            max,
        }
    }
}

impl OperationControl for UnambiguousRepeat {
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
        let guard = matcher.search.len();

        let mut p = position;
        let mut matches = 0;
        while matches < self.max && p <= guard {
            let mut iter = self.operation.matches_iter(matcher, p);
            if let Some(n) = iter.next() {
                p = n;
                matches += 1;
            } else {
                break;
            }
        }
        if matches < self.min {
            Box::new(std::iter::empty())
        } else {
            Box::new(std::iter::once(p))
        }
    }

    fn children(&self) -> Vec<Rc<Operation>> {
        vec![self.operation.clone()]
    }
}

impl RepeatOperation for UnambiguousRepeat {
    fn child(&self) -> Rc<Operation> {
        self.operation.clone()
    }

    fn min(&self) -> usize {
        self.min
    }
}
