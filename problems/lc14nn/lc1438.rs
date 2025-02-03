//! ## Leetcode 1438. Longest Continuous Subarray With Absolute Diff Less Than or Equal to Limit
//! https://leetcode.com/problems/longest-continuous-subarray-with-absolute-diff-less-than-or-equal-to-limit/
//! - `Medium`; `Independently Solved`; `2024-06-23`;
//!
//! We can use a sliding window to answer this question. When sliding the window over the `nums` array, we can use a `BTreeMap` to count the number of each element and fairly quickly calculate the maximum and minimum number.

use std::collections::BTreeMap;

pub fn longest_subarray(nums: Vec<i32>, limit: i32) -> i32 {
    let mut nums_iter = nums.iter().copied();
    let mut ans_max_len = 1;
    let mut curr_len = 1;

    let first_num = if let Some(first_num) = nums_iter.next() {
        first_num
    } else {
        return 0;
    };
    let mut map: BTreeMap<i32, usize> = BTreeMap::new();

    map.insert(first_num, 1);

    let mut prev_num_i: usize = 0;
    for num in nums_iter {
        map.entry(num).and_modify(|v| *v += 1).or_insert(1);
        let abs_diff = map.last_key_value().expect("Should have").0
            - map.first_key_value().expect("Should have").0;
        println!("{} {}", abs_diff, curr_len);
        if abs_diff <= limit {
            curr_len += 1;
            if curr_len > ans_max_len {
                ans_max_len = curr_len;
            }
        } else {
            match map.entry(nums[prev_num_i].clone()) {
                std::collections::btree_map::Entry::Vacant(_) => unreachable!(),
                std::collections::btree_map::Entry::Occupied(mut occ_entry) => {
                    if occ_entry.get() == &1 {
                        occ_entry.remove();
                    } else {
                        (*occ_entry.get_mut()) -= 1;
                    }
                }
            }

            prev_num_i += 1;
        }
    }
    ans_max_len
}
