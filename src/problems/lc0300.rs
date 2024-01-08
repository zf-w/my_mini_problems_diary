//! ## Leetcode 300. Longest Increasing Subsequences
//! https://leetcode.com/problems/longest-increasing-subsequence
//! - `Medium`; `Independently Solved`; `2024-01-04`;
//! 
//! Didn't figure out the "nlogn" version.

pub fn length_of_lis(nums: Vec<i32>) -> i32 {
  let len = nums.len();
  let mut dp: Vec<usize> = Vec::with_capacity(len);
  let nums_iter = nums.iter().enumerate();
  
  let mut ans = 1;
  for (i, num) in nums_iter {
    let mut curr = 1;
    for j in 0..i {
      if nums[j] < *num {
        curr = curr.max(dp[j] + 1);
      }
    }
    println!("{}", curr);
    dp.push(curr);
    ans = ans.max(curr as i32);
  }
  ans
}