use std::rc::Rc;

use icu_collections::codepointinvlist::CodePointInversionListBuilder;

use crate::{
    character_class::CharacterClass,
    op_nothing::Nothing,
    op_unambiguous_repeat::UnambiguousRepeat,
    operation::{
        Operation, OperationControl, MATCHES_ZLS_ANYWHERE, MATCHES_ZLS_AT_END,
        MATCHES_ZLS_AT_START, MATCHES_ZLS_NEVER,
    },
    re_compiler::ReCompiler,
    re_flags::ReFlags,
    re_matcher::{CaptureState, ReMatcher},
};

// A sequence of multiple pieces in a regular expression.
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

    fn get_initial_character_class(&self, case_blind: bool) -> CharacterClass {
        let mut builder = CodePointInversionListBuilder::new();
        for o in &self.operations {
            let cc = o.get_initial_character_class(case_blind);
            builder.add_set(cc.as_code_point_inversion_list());
            if o.matches_empty_string() == MATCHES_ZLS_NEVER {
                break;
            }
        }
        CharacterClass::new(builder.build())
    }

    fn optimize(&self, flags: &ReFlags) -> Rc<Operation> {
        match self.operations.len() {
            0 => Rc::new(Operation::from(Nothing)),
            1 => self.operations[0].clone(),
            _ => {
                let operations = self
                    .operations
                    .iter()
                    .enumerate()
                    .map(|(i, o)| {
                        let optimized = o.optimize(flags);
                        // we can never further optimize the last operation,
                        // as it has no adjacent operation
                        if i == self.operations.len() - 1 {
                            return optimized;
                        }
                        // now if this is a repeat operation, we may be able
                        // to optimize it further
                        if let Some(repeat_operation) = optimized.repeat_operation() {
                            // if the repeated operation is an atom or a charclass,
                            // we may be able to make an optimization
                            let repeated_operation = repeat_operation.child().clone();
                            if matches!(
                                repeated_operation.as_ref(),
                                Operation::Atom(_) | Operation::CharClass(_)
                            ) {
                                if repeat_operation.min() == repeat_operation.max() {
                                    // we can optimize this to an unambiguous repeat
                                    return Rc::new(Operation::from(UnambiguousRepeat::new(
                                        repeated_operation.clone(),
                                        repeat_operation.min(),
                                        repeat_operation.max(),
                                    )));
                                }
                                // get the adjacent operation
                                let next_operation = &self.operations[i + 1];
                                if ReCompiler::no_ambiguity(
                                    repeated_operation.as_ref(),
                                    next_operation,
                                    flags.is_case_independent(),
                                    !repeat_operation.greedy(),
                                ) {
                                    return Rc::new(Operation::from(UnambiguousRepeat::new(
                                        repeated_operation.clone(),
                                        repeat_operation.min(),
                                        repeat_operation.max(),
                                    )));
                                }
                            }
                        }
                        optimized
                    })
                    .collect();
                Rc::new(Operation::from(Sequence { operations }))
            }
        }
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

    fn contains_capturing_expressions(&self) -> bool {
        for o in &self.operations {
            if (matches!(o.as_ref(), Operation::Capture(_)) || o.contains_capturing_expressions()) {
                return true;
            }
        }
        false
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

    fn children(&self) -> Vec<Rc<Operation>> {
        self.operations.clone()
    }
}

struct SequenceIterator<'a> {
    iterators: Vec<Box<dyn Iterator<Item = usize> + 'a>>,
    operations: Vec<Rc<Operation>>,
    backtracking_limit: Option<usize>,
    matcher: &'a ReMatcher<'a>,
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
            Some(matcher.capture_state())
        } else {
            None
        };
        Self {
            iterators: vec![operations.first().unwrap().matches_iter(matcher, position)],
            operations,
            backtracking_limit: matcher.program.backtracking_limit,
            matcher,
            saved_state,
        }
    }
}

impl<'a> Iterator for SequenceIterator<'a> {
    type Item = usize;

    // Advance the current iterator if possible. Get the first match for all
    // subsequent iterators in the sequence. If we get all the way to the end
    // of the sequence, return the position in the input string that we have
    // reached. If we don't get all the way to the end of the sequence, work
    // backwards getting the next match for each term in the sequence until we
    // find a route through.
    fn next(&mut self) -> Option<Self::Item> {
        let mut counter = 0;
        // as long as there are iterators on the stack
        while !self.iterators.is_empty() {
            loop {
                // take the top of the stack
                let top = self.iterators.last_mut().unwrap();
                // take the next item from the top iterator
                if let Some(next) = top.next() {
                    self.matcher.clear_captured_groups_beyond(next);
                    // if the amount of iterators to process is equal or
                    // greater than the amount of operations in this sequence,
                    // then we return next
                    let i = self.iterators.len();
                    if i >= self.operations.len() {
                        return Some(next);
                    }
                    // otherwise we push a new iterator to the stack
                    let new_top = self.operations[i].matches_iter(self.matcher, next);
                    self.iterators.push(new_top);
                } else {
                    // continue until we have no more next in the top
                    break;
                }
            }
            // we are backtracking. pop the iterator from the stack
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
        // restore saved state
        if let Some(saved_state) = &self.saved_state {
            self.matcher.reset_state(saved_state.clone());
        }
        None
    }
}
