//! # Leetcode 2349. Design a Number Container System
//! https://leetcode.com/problems/design-a-number-container-system/
//! - `Medium`; `y2025m02d07`; `Independently Solved`; `59ms`; `56.8mb`; `1 attempt`;

use std::collections::{hash_map::Entry, BTreeSet, HashMap};

struct NumberContainers {
    index_to_number_hashmap: HashMap<i32, i32>,
    number_to_all_idxs_hashmap: HashMap<i32, BTreeSet<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumberContainers {
    fn new() -> Self {
        Self {
            index_to_number_hashmap: HashMap::new(),
            number_to_all_idxs_hashmap: HashMap::new(),
        }
    }

    fn helper_insert_index_to_number_group(
        mp: &mut HashMap<i32, BTreeSet<i32>>,
        number: i32,
        index: i32,
    ) {
        let number_entry = mp.entry(number);
        match number_entry {
            Entry::Occupied(mut occupied_entry) => {
                let set_mut_ref = occupied_entry.get_mut();
                set_mut_ref.insert(index);
            }
            Entry::Vacant(vacant_entry) => {
                let mut set = BTreeSet::new();
                set.insert(index);
                vacant_entry.insert(set);
            }
        }
    }

    fn helper_remove_index_from_number_group(
        mp: &mut HashMap<i32, BTreeSet<i32>>,
        number: i32,
        index: i32,
    ) {
        let number_entry = mp.entry(number);
        match number_entry {
            Entry::Occupied(mut occupied_entry) => {
                let set_mut_ref = occupied_entry.get_mut();
                set_mut_ref.remove(&index);
                println!("{:?}", *set_mut_ref);
            }
            Entry::Vacant(_) => unreachable!(),
        }
    }

    pub fn change(&mut self, index: i32, number: i32) {
        let entry = self.index_to_number_hashmap.entry(number);
        match entry {
            Entry::Occupied(mut occupied_entry) => {
                let prev_number_mut_ref = occupied_entry.get_mut();
                Self::helper_remove_index_from_number_group(
                    &mut self.number_to_all_idxs_hashmap,
                    *prev_number_mut_ref,
                    index,
                );
                *prev_number_mut_ref = number;
            }
            Entry::Vacant(vacant_entry) => {
                vacant_entry.insert(number);
            }
        }

        Self::helper_insert_index_to_number_group(
            &mut self.number_to_all_idxs_hashmap,
            number,
            index,
        );
    }

    pub fn find(&self, number: i32) -> i32 {
        let idxs_set_ref = if let Some(idxs_set_ref) = self.number_to_all_idxs_hashmap.get(&number)
        {
            idxs_set_ref
        } else {
            return -1;
        };

        idxs_set_ref.first().cloned().unwrap_or(-1)
    }
}
