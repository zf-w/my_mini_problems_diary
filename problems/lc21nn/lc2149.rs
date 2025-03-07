//! ## Leetcode 2149. Rearrange Array Elements by Sign
//! https://leetcode.com/problems/rearrange-array-elements-by-sign
//! - `Medium`; `Independently Solved`; `2024-02-13`;
//!
//! I wonder if there are in-place solutions.

pub fn rearrange_array(mut nums: Vec<i32>) -> Vec<i32> {
    let len = nums.len();
    let mut i: usize = 0;
    let mut j: usize = len;
    let mut copy = vec![0; len];

    for num in nums.iter() {
        if *num >= 0 {
            copy[i] = *num;
            i += 1;
        } else {
            copy[j - 1] = *num;
            j -= 1;
        }
    }
    i = 0;
    j = len;
    for (idx, num) in nums.iter_mut().enumerate() {
        if idx % 2 == 0 {
            *num = copy[i];
            i += 1;
        } else {
            *num = copy[j - 1];
            j -= 1;
        }
    }
    nums
}
