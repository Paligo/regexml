use crate::{operation::Operation, re_matcher::ReMatcher};

struct OpBackReference {
    group_nr: usize,
}

impl OpBackReference {
    pub(crate) fn new(group_nr: usize) -> Self {
        Self { group_nr }
    }
}

impl Operation for OpBackReference {
    fn matches_empty_string(&self) -> u32 {
        // no information available
        0
    }

    fn matches_iter<'a>(
        &self,
        matcher: &'a ReMatcher,
        position: usize,
    ) -> Box<dyn Iterator<Item = usize> + 'a> {
        // Get the start and end of the backref
        let s = matcher.state.borrow().start_back_ref[self.group_nr];
        let e = matcher.state.borrow().end_back_ref[self.group_nr];

        // We don't know the backref yet
        if s.is_none() || e.is_none() {
            return Box::new(std::iter::empty());
        }
        let s = s.unwrap();
        let e = e.unwrap();

        // The backref is the empty size
        if s == e {
            return Box::new(std::iter::once(position));
        }

        // Get the length of the backref
        let l = e - s;

        let search = matcher.search;
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
    }

    fn display(&self) -> String {
        format!("\\{}", self.group_nr)
    }
}
