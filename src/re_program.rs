pub(crate) struct ReFlags {}

impl ReFlags {
    pub(crate) fn is_multi_line(&self) -> bool {
        todo!()
    }

    pub(crate) fn is_case_independent(&self) -> bool {
        todo!()
    }
}

pub(crate) struct ReProgram {
    pub(crate) flags: ReFlags,
}
