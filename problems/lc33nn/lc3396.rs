//! # Leetcode 3396. Minimum Number of Operations to Make Elements in Array Distinct
//! https://leetcode.com/problems/minimum-number-of-operations-to-make-elements-in-array-distinct/
//! - `Easy`; `y2025m04d08`; `Independently Solved`; `0ms`; `2.2mb`; `1 attempt`;
//! Topics: uncategorized.

pub fn minimum_operations(nums: Vec<i32>) -> i32 {
    let mut num_flag_arr: [bool; 100] = [false; 100];
    let mut remain_len: usize = 0;
    let len = nums.len();
    for num in nums.into_iter().rev() {
        let num_idx = num as usize - 1;
        if num_flag_arr[num_idx] == true {
            break;
        }

        remain_len += 1;
        num_flag_arr[num_idx] = true;
    }

    let removed_len = len - remain_len;

    (removed_len as i32) / 3 + if removed_len % 3 == 0 { 0 } else { 1 }
}
