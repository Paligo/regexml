use std::rc::Rc;

use crate::{
    operation::{Operation, OperationControl, RepeatOperation, MATCHES_ZLS_ANYWHERE},
    re_flags::ReFlags,
    re_matcher::ReMatcher,
};

// Handle a repetition where there is no ambiguity; if the repeated
// term is matched in the string, then it cannot match anything other than
// the repeated term. It is also used when the number of occurrences is
// fixed. In this situation there will never be any need for
// backtracking, so there is no need to keep any information to support
// backtracking, and in addition, there is no distinction between greedy
// and reluctant matching. This operation is used only for a repeated
// atom or CharClass, which also means that if the repeated term matches
// then it can only match in one way; a typical example is the term "A*"
// in the regex "A*B".
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

    fn optimize(&self, flags: &ReFlags) -> Rc<Operation> {
        Rc::new(Operation::from(UnambiguousRepeat {
            operation: self.operation.optimize(flags),
            min: self.min,
            max: self.max,
        }))
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
