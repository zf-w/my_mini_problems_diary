//! ## Leetcode 3068. Find the Maximum Sum of Node Values
//! https://leetcode.com/problems/find-the-maximum-sum-of-node-values
//! - `Hard`; `Learned from Solution`; `2024-05-19`;
//!
//! Learned Solution:  https://leetcode.com/problems/find-the-maximum-sum-of-node-values/solutions/5176888/dp-recurion-memo-tabular-space-o-1-139ms-beats-98-89
//!
//! I guess due to the tree structure, it is always possible to file any two nodes without altering the rest of the nodes because there always exists a path between them, and the nodes between them will be flipped twice and not change.

pub fn maximum_value_sum(nums: Vec<i32>, k: i32, _: Vec<Vec<i32>>) -> i64 {
    let len = nums.len();
    let mut dp: [[i64; 2]; 2] = [[0; 2]; 2];
    dp[0][0] = 0;
    dp[0][1] = i64::MIN;
    let k_i64 = k as i64;
    for prev in 0..len {
        let curr = prev + 1;
        let x: i64 = nums[prev] as i64;
        dp[curr & 1usize][0] =
            i64::max(x + dp[(prev) & 1usize][0], (x ^ k_i64) + dp[(prev) & 1][1]);
        dp[curr & 1usize][1] =
            i64::max(x + dp[(prev) & 1usize][1], (x ^ k_i64) + dp[(prev) & 1][0]);
    }
    dp[len & 1][0]
}
