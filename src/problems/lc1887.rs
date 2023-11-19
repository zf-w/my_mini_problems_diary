//! ## Leetcode 1887. Reduction Operations to Make the Array Elements Equal
//! `Medium`
//! 
//! ### Description:
//! 
//! Given an integer array `nums`, your goal is to make all elements in `nums` equal. To complete one operation, follow these steps:
//! 
//! 1. Find the **largest** value in `nums`. Let its index be `i` **(0-indexed)** and its value be `largest`. If there are multiple elements with the largest value, pick the smallest `i`.
//! 2. Find the **next largest** value in `nums` **strictly smaller** than `largest`. Let its value be `nextLargest`.
//! 
//! 3. Reduce `nums[i]` to `nextLargest`.
//! 
//! Return *the number of operations to make all elements in `nums` equal*
//! 
//! ### Examples:
//! 
//! ```
//! use learn_cs::problems::lc1887;
//! let nums = vec![5, 1, 3];
//! assert_eq!(3, lc1887::reduction_operations(nums));
//! 
//! ```
//! 
//! ### Thoughts:
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