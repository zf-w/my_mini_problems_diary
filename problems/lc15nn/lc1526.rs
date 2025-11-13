//! # Leetcode 1526. Minimum Number of Increments on Subarrays to Form a Target Array
//! https://leetcode.com/problems/minimum-number-of-increments-on-subarrays-to-form-a-target-array/
//! - `Hard`; `y2025m10d30`; `Learned from Editorial`; `0ms`; `3.3mb`; `1 attempt`;
//! Topics: difference_array.

pub fn min_number_operations(target: Vec<i32>) -> i32 {
    let mut prev_num = 0;
    let mut ans_num = 0;
    for num in target {
        let diff = num - prev_num;
        if num > 0 {
            ans_num += diff;
        }
        prev_num = num;
    }
    ans_num
}
