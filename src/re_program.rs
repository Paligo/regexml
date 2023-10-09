use crate::{
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
        Self {
            operation,
            flags,
            prefix: None,
            optimization_flags: 0,
            max_parens,
            minimum_length,
            fixed_length,
        }
        // TODO optimization like in setOperation
    }

    pub(crate) fn get_backtracking_limit(&self) -> Option<usize> {
        todo!()
    }
}
