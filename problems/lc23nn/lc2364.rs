//! # Leetcode 2364. Count Number of Bad Pairs
//! https://leetcode.com/problems/count-number-of-bad-pairs/
//! - `Medium`; `y2025m02d08`; `Independently Solved`; `7ms`; `6.9mb`; `1 attempt`;

use std::collections::{hash_map::Entry, BTreeSet};

pub fn count_bad_pairs(nums: Vec<i32>) -> i64 {
    let len = nums.len();
    let total_pairs_num = (len * (len - 1) / 2) as i64;

    let mut diff_hashmap: HashMap<i32, u64> = HashMap::new();

    let mut total_good_pairs_num: i64 = 0;

    for (idx, num) in nums
        .into_iter()
        .enumerate()
        .map(|(idx, num)| -> (i32, i32) { (idx as i32, num) })
    {
        let diff = num - idx;
        let curr_diff_entry = diff_hashmap.entry(diff);
        match curr_diff_entry {
            Entry::Occupied(mut occupied_entry) => {
                let count_mut_ref = occupied_entry.get_mut();
                total_good_pairs_num += *count_mut_ref as i64;
                *count_mut_ref += 1;
            }
            Entry::Vacant(vacant_entry) => {
                vacant_entry.insert(1);
            }
        }
    }

    total_pairs_num - total_good_pairs_num
}
