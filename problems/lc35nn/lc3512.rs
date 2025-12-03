//! # Leetcode 3512. Minimum Operations to Make Array Sum Divisible by K
//! https://leetcode.com/problems/minimum-operations-to-make-array-sum-divisible-by-k/
//! - `Easy`; `y2025m11d28`; `Independently Solved`; `0ms`; `2.3mb`; `1 attempt`;
//! Topics: remainder.

pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
    nums.into_iter().sum::<i32>() % k        
}