use crate::{operation::Operation, re_matcher::ReMatcher};

struct OpChoice {
    branches: Vec<Box<dyn Operation>>,
}

impl OpChoice {
    pub(crate) fn new(branches: Vec<Box<dyn Operation>>) -> Self {
        Self { branches }
    }
}

impl Operation for OpChoice {
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
        self.branches
            .iter()
            .fold(0, |acc, branch| acc | branch.matches_empty_string())
    }

    // TODO
    // fn contains_capturing_expressions() -> bool {}

    fn matches_iter<'a>(
        &'a self,
        matcher: &'a ReMatcher<'a>,
        position: usize,
    ) -> Box<dyn Iterator<Item = usize> + 'a> {
        Box::new(ChoiceIterator::new(matcher, position, self.branches.iter()))
    }

    fn display(&self) -> String {
        todo!();
    }
}

struct ChoiceIterator<'a> {
    matcher: &'a ReMatcher<'a>,
    position: usize,
    branches: std::slice::Iter<'a, Box<dyn Operation>>,
    current_iter: Box<dyn Iterator<Item = usize> + 'a>,
    // current_op: Option<Box<dyn Operation>>,
}

impl<'a> ChoiceIterator<'a> {
    fn new(
        matcher: &'a ReMatcher<'a>,
        position: usize,
        mut branches: std::slice::Iter<'a, Box<dyn Operation>>,
    ) -> Self {
        let first_op = branches.next().unwrap();
        let current_iter = first_op.matches_iter(matcher, position);
        Self {
            matcher,
            position,
            branches,
            current_iter,
        }
    }
}

impl<'a> Iterator for ChoiceIterator<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let next = self.current_iter.next();
            if let Some(next) = next {
                return Some(next);
            } else {
                let next_op = self.branches.next();
                if let Some(next_op) = next_op {
                    self.current_iter = next_op.matches_iter(self.matcher, self.position);
                } else {
                    return None;
                }
            }
        }
    }
}
