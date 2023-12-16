//! ## Leetcode 1887. Reduction Operations to Make the Array Elements Equal
//! https://leetcode.com/problems/reduction-operations-to-make-the-array-elements-equal
//! - `Medium`; `Independently Solved`; `2023-11-19`;
//! 
//! I guess there is some recursive thinking in solving this. We need to first make a subset of numbers into the second largest number. Then, we can change all of the second-largest numbers into the smallest number. We can use sorting to make finding the subsets of numbers easier.

pub fn reduction_operations(mut nums: Vec<i32>) -> i32 {
  let len: usize = nums.len();
  nums.sort();
  let mut ans: usize = 0;
  for i in 1..len {
    if nums[i - 1] < nums[i] {
      ans += len - i;
    }
  }
  ans as i32
}