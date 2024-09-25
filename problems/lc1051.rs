//! ## Leetcode 1051. Height Checker
//! https://leetcode.com/problems/height-checker/
//! - `Easy`; `Independently Solved`; `2024-06-10`;
//!
//! We can sort the array and calculate the not-matching positions.

pub fn height_checker(heights: Vec<i32>) -> i32 {
    let mut height_copy = heights.clone();
    height_copy.sort();
    heights
        .iter()
        .zip(height_copy.iter())
        .fold(0, |ans_count: i32, (a, b)| -> i32 {
            if a != b {
                ans_count + 1
            } else {
                ans_count
            }
        })
}
