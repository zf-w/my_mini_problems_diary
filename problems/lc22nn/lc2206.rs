//! # Leetcode 2206. Divide Array Into Equal Pairs
//! https://leetcode.com/problems/divide-array-into-equal-pairs/
//! - `Easy`; `y2025m03d17`; `Independently Solved`; `0ms`; `2.2mb`; `2 attempt`;
//! Topics: uncategorized.

pub fn divide_array(nums: Vec<i32>) -> bool {
    let mut num_even_flag_arr: [bool; 501] = [true; 501];

    for num in nums {
        let num_idx = num as usize;
        let num_entry_mut_ref = &mut num_even_flag_arr[num_idx];

        *num_entry_mut_ref = !*num_entry_mut_ref;
    }

    for flag in num_even_flag_arr {
        if flag == false {
            return false;
        }
    }

    true
}
