use crate::{
    operation::{Operation, OperationControl},
    re_flags::ReFlags,
    re_matcher::ReMatcher,
};

// A back-reference in a regular expression.
#[derive(Debug, Clone)]
pub(crate) struct BackReference {
    group_nr: usize,
}

impl BackReference {
    pub(crate) fn new(group_nr: usize) -> Self {
        Self { group_nr }
    }
}

impl OperationControl for BackReference {
    fn matches_empty_string(&self) -> u32 {
        // no information available
        0
    }

    fn optimize(self, _flags: &ReFlags) -> Operation {
        Operation::from(self)
    }

    fn matches_iter<'a>(
        &self,
        matcher: &'a ReMatcher,
        position: usize,
    ) -> Box<dyn Iterator<Item = usize> + 'a> {
        // Get the start and end of the backref
        let s = matcher.start_backref(self.group_nr);
        let e = matcher.end_backref(self.group_nr);

        if let (Some(s), Some(e)) = (s, e) {
            // The backref is the empty size
            if s == e {
                return Box::new(std::iter::once(position));
            }

            // Get the length of the backref
            let l = e - s;

            // if there's not enough input left, give up
            let search = &matcher.search;
            if (position + l - 1) >= search.len() {
                return Box::new(std::iter::empty());
            }

            // Case fold the backref?
            if matcher.program.flags.is_case_independent() {
                // Compare backref to input
                for i in 0..l {
                    if !matcher.equal_case_blind(search[position + i], search[s + i]) {
                        return Box::new(std::iter::empty());
                    }
                }
            } else {
                // Compare backref to input
                for i in 0..l {
                    if search[position + i] != search[s + i] {
                        return Box::new(std::iter::empty());
                    }
                }
            }
            Box::new(std::iter::once(position + l))
        } else {
            // We don't know the backref yet
            Box::new(std::iter::empty())
        }
    }
}
