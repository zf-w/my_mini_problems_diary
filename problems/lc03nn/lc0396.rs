//! # LeetCode 396. Rotate Function
//! https://leetcode.com/problems/rotate-function/
//! - y2026m05d01; Independently Solved;

pub fn max_rotate_function(nums: Vec<i32>) -> i32 {
    let mut sum = 0;
    let mut f_out_num = 0;
    let num_vec_len = nums.len();
    let f_last_multiplier_plus_one = num_vec_len as u32;

    for (i, num) in nums.iter().cloned().enumerate() {
        f_out_num += (i as i32) * num;
        sum += num;
    }

    let sum = sum as u32;

    let mut ans_max_f_out_num = f_out_num;

    for rotate_off_num in nums.into_iter().rev().take(num_vec_len - 1) {
        f_out_num = ((f_out_num as u32) - f_last_multiplier_plus_one * (rotate_off_num as u32)
            + sum) as i32;

        ans_max_f_out_num = ans_max_f_out_num.max(f_out_num);
        // println!("{}", f_out_num);
    }

    ans_max_f_out_num
}