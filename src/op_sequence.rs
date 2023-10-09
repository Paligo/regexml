use std::rc::Rc;

use crate::{
    operation::{
        Operation, OperationControl, MATCHES_ZLS_ANYWHERE, MATCHES_ZLS_AT_END,
        MATCHES_ZLS_AT_START, MATCHES_ZLS_NEVER,
    },
    re_matcher::{CaptureState, ReMatcher},
};

pub(crate) struct Sequence {
    operations: Vec<Rc<Operation>>,
}

impl Sequence {
    fn new(operations: Vec<Rc<Operation>>) -> Self {
        Self { operations }
    }
}

impl OperationControl for Sequence {
    fn get_match_length(&self) -> Option<usize> {
        self.operations
            .iter()
            .try_fold(0, |acc, op| op.get_match_length().map(|len| acc + len))
    }

    fn get_minimum_match_length(&self) -> usize {
        self.operations
            .iter()
            .fold(0, |acc, op| acc + op.get_minimum_match_length())
    }

    fn matches_empty_string(&self) -> u32 {
        // The operation matches empty anywhere if every suboperation matches
        // empty anywhere
        let mut matches_empty_anywhere = true;
        for operation in &self.operations {
            let m = operation.matches_empty_string();
            if m == MATCHES_ZLS_NEVER {
                return MATCHES_ZLS_NEVER;
            }
            if m != MATCHES_ZLS_ANYWHERE {
                matches_empty_anywhere = false;
                break;
            }
        }
        if matches_empty_anywhere {
            return MATCHES_ZLS_ANYWHERE;
        }

        // The operation matches BOL if every suboperation matches BOL (which
        // includes the case of matching empty anywhere)
        let mut matches_bol = true;
        for operation in &self.operations {
            if (operation.matches_empty_string() & MATCHES_ZLS_AT_START) == 0 {
                matches_bol = false;
                break;
            }
        }
        if matches_bol {
            return MATCHES_ZLS_AT_START;
        }

        // The operation matches EOL if every suboperation matches EOL (which
        // includes the case of matching empty anywhere)
        let mut matches_eol = true;
        for operation in &self.operations {
            if (operation.matches_empty_string() & MATCHES_ZLS_AT_END) == 0 {
                matches_eol = false;
                break;
            }
        }
        if matches_eol {
            return MATCHES_ZLS_AT_END;
        }

        0
    }

    fn matches_iter<'a>(
        &self,
        matcher: &'a ReMatcher<'a>,
        position: usize,
    ) -> Box<dyn Iterator<Item = usize> + 'a> {
        Box::new(SequenceIterator::new(
            matcher,
            self.operations.clone(),
            position,
        ))
    }

    fn display(&self) -> String {
        self.operations
            .iter()
            .map(|op| op.display())
            .collect::<Vec<_>>()
            .join("")
    }
}

struct SequenceIterator<'a> {
    primed: bool,
    iterators: Vec<Box<dyn Iterator<Item = usize> + 'a>>,
    operations: Vec<Rc<Operation>>,
    capture_state: Option<CaptureState>,
    backtracking_limit: Option<usize>,
    matcher: &'a ReMatcher<'a>,
    position: usize,
}

impl<'a> SequenceIterator<'a> {
    fn new(matcher: &'a ReMatcher<'a>, operations: Vec<Rc<Operation>>, position: usize) -> Self {
        Self {
            primed: false,
            capture_state: None, // TODO: saved state is based on whether it contains capturing expressions
            iterators: Vec::new(),
            operations,
            backtracking_limit: matcher.program.get_backtracking_limit(),
            matcher,
            position,
        }
    }
}

impl<'a> Iterator for SequenceIterator<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.primed {
            self.iterators.push(
                self.operations
                    .first()
                    .unwrap()
                    .matches_iter(self.matcher, self.position),
            );
            self.primed = true;
        }
        let mut counter = 0;
        while !self.iterators.is_empty() {
            loop {
                let top = self.iterators.last_mut().unwrap();
                let p = top.next();
                if let Some(p) = p {
                    self.matcher.clear_captured_groups_beyond(p);
                    let i = self.iterators.len();
                    if i >= self.operations.len() {
                        return Some(p);
                    }
                    let top = self.operations[i].matches_iter(self.matcher, p);
                    self.iterators.push(top);
                } else {
                    break;
                }
            }
            self.iterators.pop();
            if let Some(backtracking_limit) = self.backtracking_limit {
                if counter > backtracking_limit {
                    // TODO: error
                    // Regex backtracking exceeded processing
                    todo!()
                }
            }
            counter += 1;
        }
        // TODO: if saved state, reset state in matcher with the saved state

        None
    }
}
