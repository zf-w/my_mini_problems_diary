//! # Leetcode 594. Longest Harmonious Subsequence
//! https://leetcode.com/problems/longest-harmonious-subsequence/
//! - `Easy`; `y2025m06d30`; `Independently Solved`; `6ms`; `2.7mb`; `1 attempt`;
//! Topics: subsequence.

pub fn find_lhs(nums: Vec<i32>) -> i32 {
    use std::collections::{hash_map::Entry, HashMap};
    let mut num_count_map: HashMap<i32, usize> = HashMap::new();

    let mut ans_max_len: usize = 0;
    for num in nums {
        let num_count = match num_count_map.entry(num) {
            Entry::Occupied(mut occupied_entry) => {
                let count_mut_ref = occupied_entry.get_mut();
                *count_mut_ref += 1;
                *count_mut_ref
            }
            Entry::Vacant(vacant_entry) => {
                vacant_entry.insert(1);
                1
            }
        };

        if let Some(next_count) = num_count_map.get(&(num + 1)).cloned() {
            ans_max_len = ans_max_len.max(next_count + num_count);
        }

        if let Some(next_count) = num_count_map.get(&(num - 1)).cloned() {
            ans_max_len = ans_max_len.max(next_count + num_count);
        }
    }

    ans_max_len as i32
}
