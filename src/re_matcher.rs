use crate::re_program::ReProgram;

pub(crate) struct ReMatcher<'a> {
    pub(crate) search: &'a str,
    pub(crate) program: ReProgram,
}

impl<'a> ReMatcher<'a> {
    pub(crate) fn matches(&self, str: &str, start: usize) -> bool {
        todo!()
    }

    pub(crate) fn get_paren_start(&self, i: usize) -> usize {
        todo!()
    }

    pub(crate) fn get_paren_end(&self, i: usize) -> usize {
        todo!()
    }
    pub(crate) fn get_paren_count(&self) -> usize {
        todo!()
    }

    pub(crate) fn get_paren(&self, i: usize) -> Option<&'a str> {
        todo!()
    }

    pub(crate) fn is_new_line(&self, i: usize) -> bool {
        todo!();
    }
}
