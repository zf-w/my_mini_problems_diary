//! # Leetcode 1800. Maximum Ascending Subarray Sum
//! https://leetcode.com/problems/maximum-ascending-subarray-sum/
//! - `Easy`; `y2025m02d04`; `Independently Solved`; `0ms`; `2.3mb`; `1 attempt`;
//!

pub fn max_ascending_sum(nums: Vec<i32>) -> i32 {
    let mut num_iter = nums.into_iter();
    let mut prev_num = num_iter.next().expect("len > 0");
    let mut ans_max_sum = prev_num;
    let mut curr_max_sum = prev_num;
    for curr_num in num_iter {
        if curr_num > prev_num {
            curr_max_sum += curr_num;
        } else {
            ans_max_sum = ans_max_sum.max(curr_max_sum);
            curr_max_sum = curr_num;
        }
        prev_num = curr_num;
    }
    ans_max_sum.max(curr_max_sum)
}
