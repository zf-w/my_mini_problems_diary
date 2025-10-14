//! # Leetcode 3349. Adjacent Increasing Subarrays Detection I
//! https://leetcode.com/problems/adjacent-increasing-subarrays-detection-i/
//! - `Easy`; `y2025m10d14`; `Independently Solved`; `2ms`; `2.2mb`; `2 attempts`;
//! Topics: subarray.

pub fn has_increasing_subarrays(nums: Vec<i32>, k: i32) -> bool {
    let k = k as usize;
    let mut prev_inc_subarr_len: usize = 0;
    let mut inc_subarr_len: usize = 1;
    let mut num_iter = nums.into_iter();
    let mut prev_num = num_iter.next().expect("len > 0");

    for num in num_iter {
        if num > prev_num {
            inc_subarr_len += 1;
        } else {
            prev_inc_subarr_len = inc_subarr_len;
            inc_subarr_len = 1;
        }
        if (inc_subarr_len >= k && prev_inc_subarr_len >= k) || inc_subarr_len >= 2 * k {
            return true;
        }
        // println!("{} {}", prev_inc_subarr_len, inc_subarr_len);
        prev_num = num;
    }
    false
}