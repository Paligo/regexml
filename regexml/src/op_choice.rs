use std::rc::Rc;

use crate::{
    operation::{Operation, OperationControl, MATCHES_ZLS_NEVER},
    re_matcher::ReMatcher,
};

#[derive(Debug)]
pub(crate) struct Choice {
    branches: Vec<Rc<Operation>>,
}

impl Choice {
    pub(crate) fn new(branches: Vec<Rc<Operation>>) -> Self {
        Self { branches }
    }
}

impl OperationControl for Choice {
    fn get_match_length(&self) -> Option<usize> {
        let mut iter = self.branches.iter();
        let fixed = iter.next().unwrap().get_match_length();
        for branch in iter {
            if branch.get_match_length() != fixed {
                return None;
            }
        }
        fixed
    }

    fn get_minimum_match_length(&self) -> usize {
        let mut iter = self.branches.iter();
        let mut min = iter.next().unwrap().get_minimum_match_length();
        for branch in iter {
            let m = branch.get_minimum_match_length();
            if m < min {
                min = m;
            }
        }
        min
    }

    fn matches_empty_string(&self) -> u32 {
        self.branches.iter().fold(0, |acc, branch| {
            acc | {
                let b = branch.matches_empty_string();
                if b != MATCHES_ZLS_NEVER {
                    acc | b
                } else {
                    acc
                }
            }
        })
    }

    fn contains_capturing_expressions(&self) -> bool {
        for o in &self.branches {
            if matches!(o.as_ref(), Operation::Capture(_)) || o.contains_capturing_expressions() {
                return true;
            }
        }
        false
    }

    fn matches_iter<'a>(
        &self,
        matcher: &'a ReMatcher<'a>,
        position: usize,
    ) -> Box<dyn Iterator<Item = usize> + 'a> {
        Box::new(ChoiceIterator::new(
            matcher,
            position,
            self.branches.clone(),
        ))
    }

    fn children(&self) -> Vec<Rc<Operation>> {
        self.branches.clone()
    }
}

struct ChoiceIterator<'a> {
    matcher: &'a ReMatcher<'a>,
    position: usize,
    branches_iter: Box<dyn Iterator<Item = Rc<Operation>> + 'a>,
    current_iter: Option<Box<dyn Iterator<Item = usize> + 'a>>,
}

impl<'a> ChoiceIterator<'a> {
    fn new(matcher: &'a ReMatcher<'a>, position: usize, branches: Vec<Rc<Operation>>) -> Self {
        Self {
            matcher,
            position,
            branches_iter: Box::new(branches.into_iter()),
            current_iter: None,
            // current_iter,
        }
    }

    fn next_branch(&mut self) -> bool {
        let next_op = self.branches_iter.next();
        if let Some(next_op) = next_op {
            self.matcher.clear_captured_groups_beyond(self.position);
            self.current_iter = Some(next_op.matches_iter(self.matcher, self.position));
            true
        } else {
            false
        }
    }
}

impl<'a> Iterator for ChoiceIterator<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(current_iter) = &mut self.current_iter {
                let next = current_iter.next();
                if let Some(next) = next {
                    return Some(next);
                } else if !self.next_branch() {
                    return None;
                }
            } else if !self.next_branch() {
                return None;
            }
        }
    }
}

// impl<'a> Iterator for ChoiceIterator<'a> {
//     type Item = usize;

//     fn next(&mut self) -> Option<Self::Item> {
//         loop {
//             if self.current_iter.is_none() {
//                 if let Some(next) = self.branches_iter.next() {
//                     self.matcher.clear_captured_groups_beyond(self.position);
//                     self.current_iter = Some(next.matches_iter(self.matcher, self.position));
//                 } else {
//                     return None;
//                 }
//             }
//             if let Some(current_iter) = &mut self.current_iter {
//                 return current_iter.next();
//             } else {
//                 self.current_iter = None;
//             }
//         }
//     }
// }

#[cfg(test)]
mod tests {
    use crate::{op_atom::Atom, Regex};

    use super::*;

    #[test]
    fn test_choice() {
        let regex = Regex::xpath(r#"a|b|c"#, "").unwrap();
    }
}
