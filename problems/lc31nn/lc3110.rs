//! ## Leetcode 3110. Score of a String
//! https://leetcode.com/problems/score-of-a-string/
//! - `Easy`; `Independently Solved`; `2024-06-01`;
//!
//! We can solve this by simulating the given process. This would be a good practice for using Rust iterators.

pub fn score_of_string(s: String) -> i32 {
    let iter_0 = s.chars();
    let mut iter_1 = s.chars();
    iter_1.next();
    fn diff_abs(c_0: char, c_1: char) -> i32 {
        (c_0 as u8 as i32 - c_1 as u8 as i32).abs()
    }
    iter_0
        .zip(iter_1)
        .fold(0, |acc: i32, (prev_c, curr_c)| -> i32 {
            acc + diff_abs(prev_c, curr_c)
        })
}
