//! ## Leetcode 446. Arithmetic Slices II - Subsequence
//! https://leetcode.com/problems/arithmetic-slices-ii-subsequence
//! - `Hard`; `Learned from previous`; `2024-01-06`;
//! 
//! At first, I tried "dynamic program" the length of those arithmetic slices, but that approach doesn't consider the case of duplicated numbers.

use std::collections::HashMap;
pub fn number_of_arithmetic_slices(nums: Vec<i32>) -> i32 {
  let mut ans = 0;
  let len = nums.len();
  let mut dp: Vec<HashMap<i64, i32>> = Vec::with_capacity(len);
  for i in 0..len {
    let curr = nums[i] as i64;
    let mut curr_dp: HashMap<i64, i32> = HashMap::new();
    for j in 0..i {
        let prev = nums[j] as i64;
        let prev_dp = &dp[j];
        let diff = curr - prev;
        let mut count = 0;
        if let Some(v) = prev_dp.get(&diff) {
            count = *v;
        }
        if let Some(v) = curr_dp.get_mut(&diff) {
            *v += count + 1;
        } else {
            curr_dp.insert(diff, count + 1);
        }
        ans += count;
    }
    dp.push(curr_dp);
  }
  ans         
}