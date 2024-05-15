use ahash::{HashMap, HashMapExt, HashSet, HashSetExt};

use crate::op_repeat::Repeat;

#[derive(Debug)]
pub(crate) struct History {
    zero_length_matches: HashMap<*const Repeat, HashSet<usize>>,
}

impl History {
    pub(crate) fn new() -> Self {
        Self {
            zero_length_matches: HashMap::new(),
        }
    }

    pub(crate) fn is_duplicate_zero_length_match(
        &mut self,
        repeat: &Repeat,
        position: usize,
    ) -> bool {
        // we take the address of the repeat operation as a cache key
        let cache_key = repeat as *const Repeat;

        let positions = self.zero_length_matches.get_mut(&cache_key);
        if let Some(positions) = positions {
            !positions.insert(position)
        } else {
            let mut positions = HashSet::new();
            positions.insert(position);
            self.zero_length_matches.insert(cache_key, positions);
            false
        }
    }
}
