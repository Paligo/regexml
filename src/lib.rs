mod case_variants;
mod character_class;
mod op_atom;
mod op_back_reference;
mod op_bol;
mod op_capture;
mod op_character_class;
mod op_choice;
mod op_end_program;
mod op_eol;
mod op_greedy_fixed;
mod op_nothing;
mod op_reluctant_fixed;
mod op_repeat;
mod op_sequence;
mod op_unambiguous_repeat;
mod operation;
mod re_compiler;
mod re_flags;
mod re_matcher;
mod re_program;
mod regex_iterator;
mod regular_expression;

use crate::re_compiler::{Error, ReCompiler};
use crate::re_flags::ReFlags;
use crate::re_matcher::ReMatcher;
use crate::re_program::ReProgram;

pub struct Regex {
    re_program: ReProgram,
}

impl Regex {
    pub fn new(re: &str) -> Result<Self, Error> {
        let re_flags = ReFlags::new("", "XP30")?;
        let case_variants = case_variants::CaseVariants::empty();
        let mut re_compiler = ReCompiler::new(re_flags, case_variants);
        let pattern = re.chars().collect();
        let re_program = re_compiler.compile(pattern)?;
        Ok(Self { re_program })
    }

    pub fn is_match(&self, haystack: &str) -> bool {
        let mut matcher = ReMatcher::new(&self.re_program);
        let search: Vec<char> = haystack.chars().collect();
        matcher.is_match(&search)
    }
}
