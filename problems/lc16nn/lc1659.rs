//! # Leetcode 1695. Maximum Erasure Value
//! https://leetcode.com/problems/maximum-erasure-value/
//! - `Medium`; `y2025m07d22`; `Independently Solved`; `15ms`; `3.3mb`; `1 attempt`;
//! Topics: sliding_window.

pub fn maximum_unique_subarray(nums: Vec<i32>) -> i32 {
    use std::collections::HashSet;
    let mut num_set: HashSet<i32> = HashSet::new();

    let mut begin_iter = nums.iter().cloned();

    let mut ans_sum = 0;
    let mut curr_sum = 0;

    for num in nums.iter().cloned() {
        if num_set.contains(&num) {
            for prev_num in begin_iter.by_ref() {
                if prev_num == num {
                    break;
                }
                curr_sum -= prev_num;
                num_set.remove(&prev_num);
            }
        } else {
            curr_sum += num;
            num_set.insert(num);
            ans_sum = ans_sum.max(curr_sum);
        }
    }
    ans_sum
}