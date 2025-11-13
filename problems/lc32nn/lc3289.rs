//! # Leetcode 3289. The Two Sneaky Numbers of Digitville
//! https://leetcode.com/problems/the-two-sneaky-numbers-of-digitville/
//! - `Easy`; `y2025m10d31`; `Independently Solved`; `0ms`; `2.3mb`; `1 attempt`;
//! Topics: uncategorized.

pub fn get_sneaky_numbers(nums: Vec<i32>) -> Vec<i32> {
    let mut num_set = 0u128;
    let mut ans_vec: Vec<i32> = Vec::with_capacity(2);
    for num in nums {
        let entry_bitmask: u128 = 1 << num;
        if num_set & entry_bitmask > 0 {
            ans_vec.push(num);
        }

        num_set |= entry_bitmask;
    }
    ans_vec
}