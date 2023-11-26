//! ## Leetcode 1685. Sum of Absolute Differences in a Sorted Array
//! https://leetcode.com/problems/sum-of-absolute-differences-in-a-sorted-array/
//! 
//! - `Medium`
//! 
//! You are given an integer array `nums` sorted in **non-decreasing** order.
//! 
//! Build and return *an integer array `result` with the same length as `nums` such that `result[i]` is equal to the **summation of absolute differences** between `nums[i]` and all the other elements in the array.
//! 
//! In other words, `result[i]` is equal to `sum(|nums[i] - nums[j]|)` where `0 <= j < nums.len` and `j != i` **(0-indexed)**.
//! 
//! ### Example:
//! ```
//! use learn_cs::problems::lc1685;
//! let nums = vec![2,3,5];
//! let expected = vec![4,3,5];
//! 
//! assert_eq!(expected, lc1685::get_sum_absolute_differences(nums));
//! ```
//! 
//! ### Thoughts:
//! - `Independently Solved`; `2023-11-23`;
//! 
//! Imagine you know the first element and its sum of absolute differences with the rest of elements larger than it; how will the sum of absolute differences change when you get to the next element? The sum of absolute differences would decrease in the number of elements and increase in the absolute difference between the next element and the first element.


pub fn get_sum_absolute_differences(nums: Vec<i32>) -> Vec<i32> {
    let len = nums.len();
    let mut low_count: usize = 0;
    let mut low_abs: i32 = 0;
    let mut high_count: usize = len - 1;
    let mut high_abs: i32 = 0;
    let mut ans: Vec<i32> = Vec::with_capacity(len);
    for i in 1..len {
      high_abs += nums[i] - nums[0];
    }
    ans.push(high_abs);

    for i in 1..len {
      let diff = nums[i] - nums[i - 1];
      low_count += 1;
      low_abs += diff * low_count as i32;
      high_abs -= diff * high_count as i32;
      high_count -= 1;
      ans.push(low_abs + high_abs);
    }
    ans
}