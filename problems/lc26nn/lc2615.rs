//! # LeetCode 2615. Sum of Distances
//! https://leetcode.com/problems/sum-of-distances/
//! - y2026m04d23; Hinted; 

pub fn distance(nums: Vec<i32>) -> Vec<i64> {
        use std::collections::{hash_map::Entry, HashMap};
    let len = nums.len();

    let mut num_to_group_vec_map: HashMap<i32, Vec<usize>> = HashMap::with_capacity(len);

    for (i, num) in nums.into_iter().enumerate() {
        match num_to_group_vec_map.entry(num) {
            Entry::Occupied(mut occupied_entry) => {
                let group_vec_mut_ref = occupied_entry.get_mut();
                group_vec_mut_ref.push(i);
            }
            Entry::Vacant(vacant_entry) => {
                vacant_entry.insert(vec![i]);
            }
        }
    }

    let mut ans_vec: Vec<i64> = vec![0; len];

    let idx_ref_to_i64_closure = |idx: &usize| -> i64 { *idx as i64 };

    for group_vec_ref in num_to_group_vec_map.values() {
        let total_sum = group_vec_ref
            .iter()
            .map(idx_ref_to_i64_closure)
            .sum::<i64>();

        let mut prefix_sum = 0;

        // println!("{:?}", group_vec_ref);
        let group_len_i64 = group_vec_ref.len() as i64;

        for (idx, num_idx) in group_vec_ref.iter().cloned().enumerate() {
            let num_idx_i64 = num_idx as i64;
            prefix_sum += num_idx_i64;
            let group_idx_i64 = idx as i64;

            let ans_value = (total_sum - prefix_sum)
                - (group_len_i64 - group_idx_i64 - 1) * num_idx_i64
                + (group_idx_i64 + 1) * num_idx_i64
                - prefix_sum;

            ans_vec[num_idx] = ans_value;
        }
    }

    ans_vec
}