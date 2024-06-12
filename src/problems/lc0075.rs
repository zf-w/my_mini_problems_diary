//! Leetcode 75. Sort Colors
//! https://leetcode.com/problems/sort-colors/
//! - `Medium`; `Independently Solved`; `2024-06-12`;
//!
//! Since the numbers are either `0`, `1`, or `2`, we can make the insertion quicker by keeping track of the end of previous `0`s and `1`s.

pub fn sort_colors(nums: &mut Vec<i32>) {
    let mut begin_1: usize = 0;
    let mut begin_2: usize = 0;
    let len = nums.len();
    for i in 0..len {
        let curr = nums[i];
        if curr == 2 {
            continue;
        } else if curr == 1 {
            nums[i] = 2;
            nums[begin_2] = 1;
            begin_2 += 1;
        } else if curr == 0 {
            nums[i] = 2;
            nums[begin_2] = 1;
            begin_2 += 1;
            nums[begin_1] = 0;
            begin_1 += 1;
        } else {
            unreachable!();
        }
    }
}
