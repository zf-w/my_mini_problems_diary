//! # Leetcode 3350. Adjacent Increasing Subarrays Detection II
//! https://leetcode.com/problems/adjacent-increasing-subarrays-detection-ii/
//! - `Medium`; `y2025m10d14`; `Independently Solved`; `28ms`; `4.7mb`; `1 attempt`;
//! Topics: subarray.

pub fn max_increasing_subarrays(nums: Vec<i32>) -> i32 {
    let mut prev_inc_subarr_len: usize = 0;
    let mut inc_subarr_len: usize = 1;
    let mut num_iter = nums.into_iter();
    let mut prev_num = num_iter.next().expect("len > 0");

    let mut ans_max = 0;

    for num in num_iter {
        if num > prev_num {
            inc_subarr_len += 1;
        } else {
            prev_inc_subarr_len = inc_subarr_len;
            inc_subarr_len = 1;
        }
       
        ans_max = ans_max.max(
            (inc_subarr_len.min(prev_inc_subarr_len) as i32).max(inc_subarr_len as i32 / 2),
        );
        
        // println!("{} {}", prev_inc_subarr_len, inc_subarr_len);
        prev_num = num;
    }
    ans_max
}