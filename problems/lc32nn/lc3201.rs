//! # Leetcode 3201. Find the Maximum Length of Valid Subsequence I
//! https://leetcode.com/problems/find-the-maximum-length-of-valid-subsequence-i/
//! - `Medium`; `y2025m07d16`; `Independently Solved`; `3ms`; `4.9mb`; `1 attempt`;
//! Topics: dynamic_programming.

pub fn maximum_length(nums: Vec<i32>, k: i32) -> i32 {
    let k_usize = k as usize;
    let mut dp_arr_box = vec![0; k_usize].into_boxed_slice();

    let mut ans_len = 0;

    for target in 0..k {
        for num in nums.iter().cloned().map(|v| -> i32 { v % k }) {
            let curr_len = dp_arr_box[((k + target - num) % k) as usize] + 1;
            ans_len = ans_len.max(curr_len);
            dp_arr_box[num as usize] = dp_arr_box[num as usize].max(curr_len);
        }
        for entry_mut_ref in dp_arr_box.iter_mut() {
            *entry_mut_ref = 0;
        }
    }
    ans_len as i32
}