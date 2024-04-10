use std::rc::Rc;

use crate::{
    character_class::CharacterClass,
    op_repeat::Repeat,
    operation::{Operation, OperationControl, RepeatOperation},
    re_flags::ReFlags,
};

pub(crate) const OPT_HASBACKREFS: u32 = 1;
pub(crate) const OPT_HASBOL: u32 = 2;

#[derive(Debug)]
pub(crate) struct RegexPrecondition {
    pub(crate) operation: Rc<Operation>,
    pub(crate) fixed_position: Option<usize>,
    pub(crate) min_position: usize,
}

#[derive(Debug)]
pub(crate) struct ReProgram {
    pub(crate) operation: Rc<Operation>,
    pub(crate) flags: ReFlags,
    pub(crate) prefix: Option<Vec<char>>,
    pub(crate) initial_char_class: Option<CharacterClass>,
    pub(crate) preconditions: Vec<RegexPrecondition>,
    pub(crate) minimum_length: usize,
    pub(crate) fixed_length: Option<usize>,
    pub(crate) optimization_flags: u32,
    pub(crate) max_parens: Option<usize>,
    pub(crate) backtracking_limit: Option<usize>,
}

impl ReProgram {
    pub(crate) fn new(operation: Rc<Operation>, max_parens: Option<usize>, flags: ReFlags) -> Self {
        let minimum_length = operation.get_minimum_match_length();
        let fixed_length = operation.get_match_length();

        let mut prefix = None;
        let mut optimization_flags = 0;
        let mut initial_char_class = None;

        // TODO: optimize()

        let precondition_operation = if let Operation::Sequence(op) = operation.as_ref() {
            let first = op.operations.first().unwrap();
            match first.as_ref() {
                Operation::Bol(_) => {
                    optimization_flags |= OPT_HASBOL;
                }
                Operation::Atom(atom) => prefix = Some(atom.atom.clone()),
                Operation::CharClass(char_class) => {
                    initial_char_class = Some(char_class.character_class.clone());
                }
                _ => {}
            }
            Some(operation.clone())
        } else {
            None
        };

        let mut r = Self {
            operation,
            flags,
            prefix,
            initial_char_class,
            preconditions: Vec::new(),
            optimization_flags,
            max_parens,
            minimum_length,
            fixed_length,
            backtracking_limit: None,
        };
        if let Some(precondition_operation) = precondition_operation {
            r.add_precondition(precondition_operation, None, 0);
        }
        r
    }

    pub(crate) fn initial_character_class(&self) -> Option<&CharacterClass> {
        if let Operation::Sequence(op) = self.operation.as_ref() {
            let first = op.operations.first().unwrap();
            if let Operation::CharClass(op) = first.as_ref() {
                Some(&op.character_class)
            } else {
                None
            }
        } else {
            None
        }
    }

    pub(crate) fn add_precondition(
        &mut self,
        op: Rc<Operation>,
        fixed_position: Option<usize>,
        min_position: usize,
    ) {
        match op.as_ref() {
            Operation::Atom(_) | Operation::CharClass(_) => {
                self.preconditions.push(RegexPrecondition {
                    operation: op.clone(),
                    fixed_position,
                    min_position,
                })
            }
            Operation::Repeat(repeat) if repeat.min >= 1 => {
                self.add_repeat_precondition(op.clone(), repeat, fixed_position, min_position)
            }
            Operation::ReluctantFixed(repeat) if repeat.min >= 1 => {
                self.add_repeat_precondition(op.clone(), repeat, fixed_position, min_position)
            }
            Operation::GreedyFixed(repeat) if repeat.min >= 1 => {
                self.add_repeat_precondition(op.clone(), repeat, fixed_position, min_position)
            }
            Operation::UnambiguousRepeat(repeat) if repeat.min >= 1 => {
                self.add_repeat_precondition(op.clone(), repeat, fixed_position, min_position)
            }
            Operation::Capture(capture) => {
                self.add_precondition(capture.child_op.clone(), fixed_position, min_position)
            }
            Operation::Sequence(sequence) => {
                let mut fp = fixed_position;
                let mut mp = min_position;
                for o in &sequence.operations {
                    if matches!(o.as_ref(), Operation::Bol(_)) {
                        fp = Some(0);
                    }
                    self.add_precondition(o.clone(), fp, mp);
                    if let (Some(some_fp), Some(match_length)) = (fp, o.get_match_length()) {
                        fp = Some(some_fp + match_length);
                    } else {
                        fp = None;
                    }
                    mp += o.get_minimum_match_length();
                }
            }
            _ => {}
        }
    }

    fn add_repeat_precondition<R: RepeatOperation>(
        &mut self,
        op: Rc<Operation>,
        repeat: &R,
        fixed_position: Option<usize>,
        min_position: usize,
    ) {
        let child = &repeat.child();
        match child.as_ref() {
            Operation::Atom(_) | Operation::CharClass(_) => {
                if repeat.min() == 1 {
                    self.preconditions.push(RegexPrecondition {
                        operation: op.clone(),
                        fixed_position,
                        min_position,
                    })
                } else {
                    let repeat = Operation::from(Repeat::new(
                        child.clone(),
                        repeat.min(),
                        repeat.min(),
                        true,
                    ));
                    self.preconditions.push(RegexPrecondition {
                        operation: Rc::new(repeat),
                        fixed_position,
                        min_position,
                    });
                }
            }
            _ => {
                self.add_precondition(child.clone(), fixed_position, min_position);
            }
        }
    }
}
