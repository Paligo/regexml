#![warn(missing_docs)]
#![doc = include_str!("../README.md")]

mod analyze_string;
mod block;
mod category;
mod character_class;
mod history;
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
mod regex;

pub use crate::analyze_string::{AnalyzeEntry, MatchEntry};
pub use crate::re_compiler::Error;
pub use crate::regex::Regex;
