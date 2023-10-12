use std::rc::Rc;

use crate::{
    operation::{
        Operation, OperationControl, MATCHES_ZLS_ANYWHERE, MATCHES_ZLS_AT_END,
        MATCHES_ZLS_AT_START, MATCHES_ZLS_NEVER,
    },
    re_matcher::{CaptureState, ReMatcher},
};

#[derive(Debug)]
pub(crate) struct Sequence {
    pub(crate) operations: Vec<Rc<Operation>>,
}

impl Sequence {
    pub(crate) fn new(operations: Vec<Rc<Operation>>) -> Self {
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
            self.contains_capturing_expressions(),
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
    backtracking_limit: Option<usize>,
    matcher: &'a ReMatcher<'a>,
    position: usize,
    saved_state: Option<CaptureState>,
}

impl<'a> SequenceIterator<'a> {
    fn new(
        matcher: &'a ReMatcher<'a>,
        operations: Vec<Rc<Operation>>,
        position: usize,
        contains_capturing_expressions: bool,
    ) -> Self {
        let saved_state = if contains_capturing_expressions {
            Some(matcher.state.borrow().capture_state.clone())
        } else {
            None
        };

        Self {
            primed: false,
            iterators: Vec::new(),
            operations,
            backtracking_limit: matcher.program.backtracking_limit,
            matcher,
            position,
            saved_state,
        }
    }

    // fn contains_capturing_expressions(&self) -> bool {
    //     for o in &self.operations {
    //         if matches!(o.as_ref(), Operation::Capture(_)) || o.contains_capturing_expressions() {
    //             return true;
    //         }
    //     }
    //     false
    // }
}

impl<'a> Iterator for SequenceIterator<'a> {
    type Item = usize;

    // Get the first match for all subsequent iterators in the sequence. If we
    // get all the way to the end of the sequence, return the position in the
    // input string that we have reached. If we don't get all the way to the
    // end of the sequence, work backwards getting the next match for each term
    // in the sequence until we find a route through.
    fn next(&mut self) -> Option<Self::Item> {
        // TODO: can this move to constructor?
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
                    let new_top = self.operations[i].matches_iter(self.matcher, p);
                    self.iterators.push(new_top);
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
        if let Some(saved_state) = self.saved_state.take() {
            self.matcher.reset_state(saved_state);
        }
        None
    }
}
