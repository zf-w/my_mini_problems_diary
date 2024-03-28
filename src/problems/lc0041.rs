//! ## Leetcode 41. First Missing Positive
//! https://leetcode.com/problems/first-missing-positive
//! - `Hard`; `Independently Solved`; `2024-03-25`;
//!
//! This problem continues the spirit of yesterday's problem: how to use the available space to store the presence of positive integers.

pub fn first_missing_positive(mut nums: Vec<i32>) -> i32 {
    let len = nums.len();
    let len_i32 = len as i32;
    for num_i in 0..len {
        let curr = nums[num_i];
        if curr <= 0 || curr > len_i32 {
            nums[num_i] = 0;
        }
    }
    for num_i in 0..len {
        let curr = nums[num_i];
        if curr == i32::MIN || curr == 0 {
            continue;
        }

        let curr_i = curr.abs() as usize - 1;
        let curr_entry = &mut nums[curr_i];
        // print!("{curr_i} Before: {}", nums[curr_i]);
        if *curr_entry > 0 {
            *curr_entry = -(curr_entry.abs());
        } else if *curr_entry == 0 {
            *curr_entry = i32::MIN;
        }
        // println!("After: {}", nums[curr_i]);
    }
    for num_i in 0..len {
        // print!("{} ", nums[num_i]);
        if nums[num_i] >= 0 {
            return num_i as i32 + 1;
        }
    }
    len_i32 + 1
}
