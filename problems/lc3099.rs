//! ## Leetcode 3099. Harshad Number
//! https://leetcode.com/problems/harshad-number
//! - `Easy`; `Independently Solved`; `2024-03-30`;
//!
//! A simulation approach would work.

pub fn sum_of_the_digits_of_harshad_number(x: i32) -> i32 {
    let mut digit_sum = 0;
    let mut copy = x.clone();
    while copy > 0 {
        digit_sum += copy % 10;
        copy /= 10;
    }
    if x % digit_sum == 0 {
        digit_sum
    } else {
        -1
    }
}
