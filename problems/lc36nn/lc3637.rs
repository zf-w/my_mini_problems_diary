//! # Leetcode 3637. Trionic Array I
//! https://leetcode.com/problems/trionic-array-i/
//! y2026m02d03; Independently Solved; 

pub fn is_trionic(nums: Vec<i32>) -> bool {
    let mut num_iter = nums.into_iter();
    let mut prev_num = num_iter.next().expect("len > 0");

    let mut state_idx = 0;

    for num in num_iter {
        if num > prev_num {
            if state_idx == 0 {
                state_idx = 1;
            } else if state_idx == 1 {
            } else if state_idx == 2 {
                state_idx = 3;
            } else if state_idx == 3 {
            }
        } else if num == prev_num {
            return false;
        } else {
            if state_idx == 0 {
                return false;
            } else if state_idx == 1 {
                state_idx = 2;
            } else if state_idx == 2 {
            } else if state_idx == 3 {
                state_idx = 4;
            }
        }
        prev_num = num;
    }

    state_idx == 3
}