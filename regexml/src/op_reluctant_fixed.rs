use crate::{
    operation::{Operation, OperationControl, RepeatOperation, MATCHES_ZLS_ANYWHERE},
    re_flags::ReFlags,
    re_matcher::ReMatcher,
};

// Handle a reluctant repetition (with possible min and max) where the size of
// the repeated unit is fixed.
#[derive(Debug, Clone)]
pub(crate) struct ReluctantFixed {
    operation: Box<Operation>,
    pub(crate) min: usize,
    max: usize,
    len: usize,
}

impl ReluctantFixed {
    pub(crate) fn new(operation: Operation, min: usize, max: usize, len: usize) -> Self {
        Self {
            operation: operation.into(),
            min,
            max,
            len,
        }
    }
}

impl OperationControl for ReluctantFixed {
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

    fn contains_capturing_expressions(&self) -> bool {
        matches!(self.operation.as_ref(), Operation::Capture(_))
            || self.operation.contains_capturing_expressions()
    }

    fn optimize(self, flags: &ReFlags) -> Operation {
        let operation = self.operation.optimize(flags);
        Operation::from(ReluctantFixed {
            operation: operation.into(),
            min: self.min,
            max: self.max,
            len: self.len,
        })
    }

    fn matches_iter<'a>(
        &'a self,
        matcher: &'a ReMatcher,
        position: usize,
    ) -> Box<dyn Iterator<Item = usize> + 'a> {
        Box::new(ReluctantFixedIterator::new(
            self.operation.as_ref(),
            matcher,
            position,
            self.min,
            self.max,
        ))
    }

    fn children(&self) -> Vec<Operation> {
        vec![self.operation.as_ref().clone()]
    }
}

impl RepeatOperation for ReluctantFixed {
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
        false
    }
}

struct ReluctantFixedIterator<'a> {
    op: &'a Operation,
    matcher: &'a ReMatcher<'a>,
    position: usize,
    count: usize,
    min: usize,
    max: usize,
    started: bool,
    pos: usize,
}

impl<'a> ReluctantFixedIterator<'a> {
    fn new(
        op: &'a Operation,
        matcher: &'a ReMatcher<'a>,
        position: usize,
        min: usize,
        max: usize,
    ) -> Self {
        Self {
            op,
            matcher,
            position,
            count: 0,
            min,
            max,
            started: false,
            pos: position,
        }
    }
}

impl<'a> Iterator for ReluctantFixedIterator<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.started {
            self.started = true;

            while self.count < self.min {
                let mut it = self.op.matches_iter(self.matcher, self.pos);
                if let Some(next) = it.next() {
                    self.count += 1;
                    self.pos = next;
                } else {
                    return None;
                }
            }
            return Some(self.pos);
        }

        if self.count < self.max {
            self.matcher.clear_captured_groups_beyond(self.position);
            let mut it = self.op.matches_iter(self.matcher, self.pos);
            if let Some(next) = it.next() {
                self.pos = next;
                self.count += 1;
                return Some(self.pos);
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::Regex;

    #[test]
    fn test_repeat_star_relucant() {
        let regex = Regex::xpath(r#"a*?"#, "").unwrap();
        let op = regex.path("0");
        let matcher = regex.matcher("aaaaa");
        let matches = matcher.operation_matches(op);
        assert_eq!(matches, vec!["", "a", "aa", "aaa", "aaaa", "aaaaa"]);
    }

    #[test]
    fn test_repeat_plus_reluctant() {
        let regex = Regex::xpath(r#"a+?"#, "").unwrap();
        let op = regex.path("0");
        let matcher = regex.matcher("aaaaa");
        let matches = matcher.operation_matches(op);
        assert_eq!(matches, vec!["a", "aa", "aaa", "aaaa", "aaaaa"]);
    }

    #[test]
    fn test_repeat_question_reluctant() {
        let regex = Regex::xpath(r#"a??"#, "").unwrap();
        let op = regex.path("0");
        let matcher = regex.matcher("aaaaa");
        let matches = matcher.operation_matches(op);
        assert_eq!(matches, vec!["", "a"]);
    }
}
