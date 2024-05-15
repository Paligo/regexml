use ahash::{HashMap, HashMapExt, HashSet, HashSetExt};

use crate::operation::Operation;

#[derive(Debug)]
pub(crate) struct History {
    zero_length_matches: HashMap<Operation, HashSet<usize>>,
}

impl History {
    pub(crate) fn new() -> Self {
        Self {
            zero_length_matches: HashMap::new(),
        }
    }

    pub(crate) fn is_duplicate_zero_length_match(
        &mut self,
        operation: Operation,
        position: usize,
    ) -> bool {
        // TODO: hashing an operation; how can that work with enum dispatch?
        let positions = self.zero_length_matches.get_mut(&operation);
        if let Some(positions) = positions {
            if positions.contains(&position) {
                true
            } else {
                positions.insert(position);
                false
            }
        } else {
            let mut positions = HashSet::new();
            positions.insert(position);
            self.zero_length_matches.insert(operation, positions);
            false
        }
    }
}
