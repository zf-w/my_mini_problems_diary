//! ## Leetcode 1043. Partition Array for Maximum Sum
//! https://leetcode.com/problems/partition-array-for-maximum-sum
//! - `Medium`; `Independently Solved`; `2024-02-03`;
//!
//! At first, I was thinking about sorting and picking the top numbers and distributing those into groups.
//! Later, I found this question is about partition instead of swapping numbers.

pub fn max_sum_after_partitioning(arr: Vec<i32>, k: i32) -> i32 {
    let len = arr.len();
    let ku = k as usize;
    let mut dp: Vec<i32> = Vec::with_capacity(len);
    dp.push(0);
    for i in 0..len {
        let curr_val = arr[i];
        let mut curr_dp = 0;
        let mut curr_max = curr_val;
        for back in 1..=(ku.min(i + 1)) {
            let j = i + 1 - back;
            curr_max = curr_max.max(arr[j]);
            curr_dp = curr_dp.max(dp[j] + curr_max * (back) as i32);
        }
        dp.push(curr_dp);
    }
    dp[len]
}
