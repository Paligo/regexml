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
    pub(crate) operation: Operation,
    pub(crate) fixed_position: Option<usize>,
    pub(crate) min_position: usize,
}

#[derive(Debug)]
pub(crate) struct ReProgram {
    pub(crate) pattern: Vec<char>,
    pub(crate) operation: Operation,
    pub(crate) flags: ReFlags,
    pub(crate) prefix: Option<Vec<char>>,
    pub(crate) initial_char_class: Option<CharacterClass>,
    pub(crate) preconditions: Vec<RegexPrecondition>,
    pub(crate) minimum_length: usize,
    pub(crate) optimization_flags: u32,
    pub(crate) max_parens: Option<usize>,
    pub(crate) backtracking_limit: Option<usize>,
}

impl ReProgram {
    pub(crate) fn new(
        pattern: Vec<char>,
        operation: Operation,
        max_parens: Option<usize>,
        flags: ReFlags,
    ) -> Self {
        let minimum_length = operation.get_minimum_match_length();

        let mut prefix = None;
        let mut optimization_flags = 0;
        let mut initial_char_class = None;

        let precondition_operation = if let Operation::Sequence(op) = operation.clone() {
            let first = op.operations.first().unwrap();
            match first {
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
            pattern,
            operation,
            flags,
            prefix,
            initial_char_class,
            preconditions: Vec::new(),
            optimization_flags,
            max_parens,
            minimum_length,
            backtracking_limit: None,
        };
        if let Some(precondition_operation) = precondition_operation {
            r.add_precondition(precondition_operation, None, 0);
        }
        r
    }

    pub(crate) fn add_precondition(
        &mut self,
        op: Operation,
        fixed_position: Option<usize>,
        min_position: usize,
    ) {
        match &op {
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
            Operation::Capture(capture) => self.add_precondition(
                capture.child_op.as_ref().clone(),
                fixed_position,
                min_position,
            ),
            Operation::Sequence(sequence) => {
                let mut fp = fixed_position;
                let mut mp = min_position;
                for o in &sequence.operations {
                    if matches!(o, Operation::Bol(_)) {
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
        op: Operation,
        repeat: &R,
        fixed_position: Option<usize>,
        min_position: usize,
    ) {
        let child = &repeat.child();
        match child {
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
                        operation: repeat,
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

    #[cfg(test)]
    pub(crate) fn path(&self, path: &str) -> Operation {
        // path is numbers separated by / and goes into the children of the operation
        // so 0/1/2 would go into the 0th child, then the 1st child of that, then the 2nd child of that
        let steps = path
            .split('/')
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<usize>().unwrap());
        let mut op = self.operation.clone();
        for step in steps {
            op = op.children()[step].clone();
        }
        op
    }
}
