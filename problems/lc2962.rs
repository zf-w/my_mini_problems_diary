//! ## Leetcode 2962. Count Subarrays Where Max Element Appears at Least K Times
//! https://leetcode.com/problems/count-subarrays-where-max-element-appears-at-least-k-times
//! - `Medium`; `Independently Solved`; `2024-03-28`;
//!
//! Another sliding window problem. I tried to memorize the position of maximum values and use multiplication to speed up, instead of using two nested loops.

pub fn count_subarrays(nums: Vec<i32>, k: i32) -> i64 {
    let len = nums.len();
    let max_v = nums.iter().max().expect("len > 1").clone();
    let mut max_v_idxs: Vec<usize> = Vec::with_capacity(len);
    for (i, num) in nums.iter().cloned().enumerate() {
        if num == max_v {
            max_v_idxs.push(i);
        }
    }
    let mut begin_i: usize = 0;
    let mut end_i: usize = k as usize;
    let max_v_len = max_v_idxs.len();
    let mut ans = 0;
    while end_i <= max_v_len && end_i > 0 {
        let next_i = if end_i == max_v_len {
            len
        } else {
            max_v_idxs[end_i]
        };
        let next_space = next_i - max_v_idxs[end_i - 1];
        ans += ((1 + max_v_idxs[begin_i]) * next_space) as i64;
        // println!("{}", ans);
        begin_i += 1;
        end_i += 1;
    }
    ans
}
