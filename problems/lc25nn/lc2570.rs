//! # Leetcode 2570. Merge Two 2D arrays by Summing Values
//! https://leetcode.com/problems/merge-two-2d-arrays-by-summing-values/
//! - `Easy`; `y2025m03d02`; `Independently Solved`; `2ms`; `2.4mb`; `1 attempt`;
//! Topics: uncategorized.

pub fn merge_arrays(nums1: Vec<Vec<i32>>, nums2: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    use std::collections::{btree_map::Entry, BTreeMap};
    let mut mp: BTreeMap<i32, i32> = BTreeMap::new();
    for pair_vec in nums1.into_iter().chain(nums2.into_iter()) {
        let key = pair_vec[0];
        let val = pair_vec[1];
        match mp.entry(key) {
            Entry::Vacant(vacant_entry) => {
                vacant_entry.insert(val);
            }
            Entry::Occupied(mut occupied_entry) => {
                *(occupied_entry.get_mut()) += val;
            }
        }
    }

    let mut ans_vec_vec: Vec<Vec<i32>> = Vec::with_capacity(mp.len());
    for (key, val) in mp {
        ans_vec_vec.push(vec![key, val]);
    }
    ans_vec_vec
}
