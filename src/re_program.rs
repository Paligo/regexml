pub(crate) const OPT_HASBACKREFS: u32 = 1;
pub(crate) const OPT_HASBOL: u32 = 2;

pub(crate) struct ReFlags {}

impl ReFlags {
    pub(crate) fn is_multi_line(&self) -> bool {
        todo!()
    }

    pub(crate) fn is_case_independent(&self) -> bool {
        todo!()
    }

    pub(crate) fn is_single_line(&self) -> bool {
        todo!()
    }
}

pub(crate) struct ReProgram {
    pub(crate) flags: ReFlags,
    pub(crate) optimization_flags: u32,
}

impl ReProgram {
    pub(crate) fn get_backtracking_limit(&self) -> Option<usize> {
        todo!()
    }
}
