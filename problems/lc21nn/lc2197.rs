//! # Leetcode 2197. Replace Non-Coprime Numbers in Array
//! https://leetcode.com/problems/replace-non-coprime-numbers-in-array/
//! - `Hard`; `y2025m09d16`; `Learned from Solution`; `8ms`; `3.7mb`; `1 attempt`;
//! Topics: coprime.
//! Learned from solution: https://leetcode.com/problems/replace-non-coprime-numbers-in-array/solutions/7194484/lcm-stack-reuse-nums-beats-100

pub fn replace_non_coprimes(mut nums: Vec<i32>) -> Vec<i32> {
    fn calc_greatest_common_divisor(mut num_0: i32, mut num_1: i32) -> i32 {
        while num_1 != 0 {
            (num_0, num_1) = (num_1, num_0 % num_1)
        }

        num_0
    }

    let len = nums.len();
    let mut stk_top_i: usize = 0;
    let mut stk_empty_flag = false;
    let mut iter_i: usize = 1;

    fn stk_push(stk_top_i_mut_ref: &mut usize, stk_empty_flag_mut_ref: &mut bool) {
        if *stk_empty_flag_mut_ref == true {
            *stk_empty_flag_mut_ref = false;
        } else {
            *stk_top_i_mut_ref += 1;
        }
    }

    fn stk_pop(stk_top_i_mut_ref: &mut usize, stk_empty_flag_mut_ref: &mut bool) {
        if *stk_empty_flag_mut_ref == true {
            return;
        }

        if *stk_top_i_mut_ref == 0 {
            *stk_empty_flag_mut_ref = true;
        } else {
            *stk_top_i_mut_ref -= 1;
        }
    }

    while iter_i < len {
        let mut curr_num = nums[iter_i];
        while stk_empty_flag == false {
            let stk_top_mut_ref = &mut nums[stk_top_i];
            let common_divisor = calc_greatest_common_divisor(*stk_top_mut_ref, curr_num);
            if common_divisor == 1 {
                break;
            }
            curr_num *= *stk_top_mut_ref / common_divisor;

            stk_pop(&mut stk_top_i, &mut stk_empty_flag);
        }
        stk_push(&mut stk_top_i, &mut stk_empty_flag);
        nums[stk_top_i] = curr_num;

        iter_i += 1;
    }

    nums.resize(stk_top_i + 1, 0);
    nums
}
