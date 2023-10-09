use std::rc::Rc;

use crate::{
    operation::{Operation, OperationControl, MATCHES_ZLS_ANYWHERE},
    re_matcher::ReMatcher,
};

#[derive(Clone)]
pub(crate) struct ReluctantFixed {
    operation: Rc<Operation>,
    min: usize,
    max: usize,
    len: usize,
}

impl ReluctantFixed {
    pub(crate) fn new(operation: Rc<Operation>, min: usize, max: usize, len: usize) -> Self {
        Self {
            operation,
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

    fn matches_iter<'a>(
        &self,
        matcher: &'a ReMatcher,
        position: usize,
    ) -> Box<dyn Iterator<Item = usize> + 'a> {
        Box::new(ReluctantFixedIterator::new(
            self.operation.clone(),
            matcher,
            position,
            self.min,
            self.max,
        ))
    }

    fn display(&self) -> String {
        todo!();
    }
}

struct ReluctantFixedIterator<'a> {
    op: Rc<Operation>,
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
        op: Rc<Operation>,
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
        return None;
    }
}
