//! # Leetcode 2523. Closest Prime Numbers in Range
//! https://leetcode.com/problems/closest-prime-numbers-in-range/
//! - `Medium`; `y2025m03d06`; `Independently Solved`; `83ms`; `3.6mb`;
//! Topics: prime_num/sieve.

pub fn closest_primes(left: i32, right: i32) -> Vec<i32> {
    let begin = left as usize;
    let end = right as usize + 1;

    let mut num_flag_vec: Vec<bool> = vec![true; end];

    let mut ans_dis_opt: Option<usize> = None;
    let mut ans_pair_vec: Vec<i32> = vec![-1, -1];

    let mut prev_num_opt: Option<usize> = None;
    for num in 2..end {
        if num_flag_vec[num] == false {
            continue;
        }

        let mut num_multplier = num * 2;
        while num_multplier < end {
            num_flag_vec[num_multplier] = false;
            num_multplier += num;
        }

        if num < begin || num >= end {
            continue;
        }

        if let Some(prev_num) = prev_num_opt.as_mut() {
            let curr_dis = num - *prev_num;
            if let Some(prev_dis) = ans_dis_opt.as_mut() {
                if *prev_dis > curr_dis {
                    *prev_dis = curr_dis;
                    ans_pair_vec[0] = (*prev_num) as i32;
                    ans_pair_vec[1] = num as i32;
                }
            } else {
                ans_dis_opt = Some(curr_dis);
                ans_pair_vec[0] = (*prev_num) as i32;
                ans_pair_vec[1] = num as i32;
            }
            *prev_num = num;
        } else {
            prev_num_opt = Some(num);
        }
    }
    ans_pair_vec
}
