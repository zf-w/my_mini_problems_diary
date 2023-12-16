//! ## Leetcode 1685. Sum of Absolute Differences in a Sorted Array
//! https://leetcode.com/problems/sum-of-absolute-differences-in-a-sorted-array/
//! - `Medium`; `Independently Solved`; `2023-11-23`;
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