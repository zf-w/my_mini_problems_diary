//! # Leetcode 2780. Minimum Index of a Valid Split
//! https://leetcode.com/problems/minimum-index-of-a-valid-split/
//! - `Medium`; `y2025m03d27`; `Hinted`; `9ms`; `5.7mb`; `1 attempt`;
//! Topics: uncategorized.
//!
//! I thought I was needing a key-updatable heap or something...

pub fn minimum_index(nums: Vec<i32>) -> i32 {
    use std::collections::{hash_map::Entry, HashMap};

    let num_num = nums.len();
    let mut count_map: HashMap<i32, usize> = HashMap::with_capacity(num_num);

    for num in nums.iter().cloned() {
        match count_map.entry(num) {
            Entry::Occupied(mut occupied_entry) => {
                *occupied_entry.get_mut() += 1;
            }
            Entry::Vacant(vacant_entry) => {
                vacant_entry.insert(1);
            }
        }
    }

    let mut map_iter = count_map
        .iter()
        .map(|v| -> (i32, usize) { (v.0.clone(), v.1.clone()) });

    let (mut dom_val, mut dom_count) = map_iter.next().expect("len > 0");

    for (num, count) in map_iter {
        if count > dom_count {
            dom_count = count;
            dom_val = num;
        }
    }

    let mut prev_dom_count: usize = 0;
    let mut prev_len: usize = 0;
    let mut next_len: usize = num_num;
    for (i, num) in nums.into_iter().enumerate() {
        prev_len += 1;
        next_len -= 1;
        if num == dom_val {
            prev_dom_count += 1;
            dom_count -= 1;
            if prev_dom_count > prev_len / 2 && dom_count > next_len / 2 {
                return i as i32;
            }
        }
    }
    -1
}
