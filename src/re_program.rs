use crate::{
    character_class::CharacterClass,
    operation::{Operation, OperationControl},
    re_flags::ReFlags,
};

pub(crate) const OPT_HASBACKREFS: u32 = 1;
pub(crate) const OPT_HASBOL: u32 = 2;

pub(crate) struct ReProgram {
    pub(crate) operation: Operation,
    pub(crate) flags: ReFlags,
    pub(crate) prefix: Option<Vec<char>>,
    pub(crate) minimum_length: usize,
    pub(crate) fixed_length: Option<usize>,
    pub(crate) optimization_flags: u32,
    pub(crate) max_parens: Option<usize>,
}

impl ReProgram {
    pub(crate) fn new(operation: Operation, max_parens: Option<usize>, flags: ReFlags) -> Self {
        let minimum_length = operation.get_minimum_match_length();
        let fixed_length = operation.get_match_length();

        let mut prefix = None;
        let mut optimization_flags = 0;

        // TODO: optimize()

        if let Operation::Sequence(op) = &operation {
            let first = op.operations.first().unwrap();
            match first.as_ref() {
                Operation::Bol(_) => {
                    optimization_flags |= OPT_HASBOL;
                }
                Operation::Atom(atom) => prefix = Some(atom.atom.clone()),
                // Operation::CharClass(op) => initial_character_class = { todo!(); //
                //     // get initial character class dynamically from pr
                //     Some(op.character_class),
                // }
                _ => {}
            }
            // r.add_precondition(Operation::from(op), None, 0);
        }

        Self {
            operation,
            flags,
            prefix,
            optimization_flags,
            max_parens,
            minimum_length,
            fixed_length,
        }
    }

    pub(crate) fn initial_character_class(&self) -> Option<&CharacterClass> {
        if let Operation::Sequence(op) = &self.operation {
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

    pub(crate) fn get_backtracking_limit(&self) -> Option<usize> {
        todo!()
    }

    pub(crate) fn add_precondition(
        &mut self,
        op: Operation,
        fixed_position: Option<usize>,
        min_position: usize,
    ) {
        todo!()
    }
}
