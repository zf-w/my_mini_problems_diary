//! # Leetcode 1749. Maximum Absolute Sum of Any Subarray
//! https://leetcode.com/problems/maximum-absolute-sum-of-any-subarray/
//! - `Medium`; `y2025m02d25`; `Independently Solved`; `0ms`; `3.1mb`; `1 attempt`;
//!
//! Topics: subarray.

pub fn max_absolute_sum(nums: Vec<i32>) -> i32 {
    let mut min_sum = 0;
    let mut max_sum = 0;
    let mut ans = 0;
    for num in nums {
        min_sum = (min_sum + num).min(0);
        max_sum = (max_sum + num).max(0);
        ans = ans.max(min_sum.abs().max(max_sum)); // haha. Was `ans = min_sum.abs().max(max_sum);`
    }
    ans
}
