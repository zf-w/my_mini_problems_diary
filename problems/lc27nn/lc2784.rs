//! # LeetCode 2784. Check if Array is Good
//! https://leetcode.com/problems/check-if-array-is-good/
//! - y2026m05d14; Independently Solved 

pub fn is_good(nums: Vec<i32>) -> bool {
    let len = nums.len();
    let mut remaining_count = len;
    let last_num = (len - 1) as i32;
    let mut flag_vec = vec![false; len];
    for num in nums {
        let num_i = (num - 1) as usize;
        
        if num > last_num {
            return false;
        }

        if num == last_num {
            let entry_mut_ref = &mut flag_vec[num_i];
            if *entry_mut_ref == true {
                let next_entry_mut_ref = &mut flag_vec[num_i + 1];
                if *next_entry_mut_ref == true {
                    return false;
                } else {
                    *next_entry_mut_ref = true;
                    remaining_count -= 1;
                }
            } else {
                *entry_mut_ref = true;
                remaining_count -= 1;
            }
        } else {
            let entry_mut_ref = &mut flag_vec[num_i];
            if *entry_mut_ref == true {
                return false;
            } else {
                *entry_mut_ref = true;
                remaining_count -= 1;
            }
        }
    }

    remaining_count == 0
}