use crate::re_program::ReProgram;

pub(crate) struct ReMatcher<'a> {
    pub(crate) search: &'a [char],
    pub(crate) program: ReProgram,
    pub(crate) start_back_ref: Vec<Option<usize>>,
    pub(crate) end_back_ref: Vec<Option<usize>>,
}

impl<'a> ReMatcher<'a> {
    pub(crate) fn matches(&self, str: &[char], start: usize) -> bool {
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

    pub(crate) fn equal_case_blind(&self, a: char, b: char) -> bool {
        todo!()
    }
}
