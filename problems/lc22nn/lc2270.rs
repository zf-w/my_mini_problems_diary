//! # Leetcode 2270. Number of Ways to Split Array
//! https://leetcode.com/problems/number-of-ways-to-split-array/
//! - `Medium`; `y2025m01d03`; `Independently Solved`; `3ms`; `3.9mb`; `4 attempts`;

pub fn ways_to_split_array(nums: Vec<i32>) -> i32 {
    let mut ans_count: i32 = 0;
    let total_sum: i64 = nums.iter().map(|v| *v as i64).sum();

    let mut left_sum: i64 = 0;
    for num in nums {
        left_sum += num as i64;
        let right_sum = total_sum - left_sum;
        if left_sum >= right_sum {
            ans_count += 1;
        }
    }
    ans_count - if left_sum >= 0 { 1 } else { 0 }
}
