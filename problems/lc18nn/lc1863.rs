//! ## Leetcode 1863. Sum of All Subset XOR Totals
//! https://leetcode.com/problems/sum-of-all-subset-xor-totals
//! - `Easy`; `Independently Solved`; `2024-05-20`;
//!
//! I would say this is more of a medium-level problem because it's related to subsets, recursion, and dynamic programming. We can directly simulate and calculate the answer by following the problem description due to the small size of the problem.

pub fn subset_xor_sum(nums: Vec<i32>) -> i32 {
    let len = nums.len();
    if len == 0 {
        return 0;
    }
    let mut dp: Vec<i32> = Vec::with_capacity(2_usize.pow(len as u32 - 1));
    let mut ans_sum = 0;

    dp.push(0);
    for num in nums.iter() {
        let dp_len = dp.len();
        for prev_dp_i in 0..dp_len {
            let curr = *num ^ dp[prev_dp_i];
            dp.push(curr);
            ans_sum += curr;
        }
    }
    ans_sum
}
