//! # Leetcode 3105. Longest Strictly Increasing or Strictly Decreasing Subarray
//! https://leetcode.com/problems/longest-strictly-increasing-or-strictly-decreasing-subarray/
//! - `Easy`; `y2025m02d03`; `Independently Solved`; `0ms`; `2.3mb`; `1 attempt`;

pub fn longest_monotonic_subarray(nums: Vec<i32>) -> i32 {
    const INIT_LEN: i32 = 1;
    let mut ans_max_len = INIT_LEN;
    let mut num_iter = nums.into_iter();
    let mut prev_num = num_iter.next().expect("len > 0");

    let mut curr_inc_len = INIT_LEN;
    let mut curr_dec_len = INIT_LEN;
    for curr_num in num_iter {
        if curr_num > prev_num {
            curr_inc_len += 1;
            curr_dec_len = INIT_LEN;
        } else if curr_num < prev_num {
            curr_dec_len += 1;
            curr_inc_len = INIT_LEN;
        } else {
            curr_inc_len = INIT_LEN;
            curr_dec_len = INIT_LEN;
        }
        ans_max_len = ans_max_len.max(curr_inc_len.max(curr_dec_len));
        prev_num = curr_num;
    }
    ans_max_len
}
