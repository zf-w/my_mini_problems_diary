//! # Leetcode 1513. Number of Substrings With Only 1s
//! https://leetcode.com/problems/number-of-substrings-with-only-1s/
//! - `Medium`; `y2025m11d16`; `Independently Solved`; `0ms`; `2.2mb`; `2 attempts`;
//! Topics: sliding_window.

pub fn num_sub(s: String) -> i32 {
    const MODULO: i32 = 1000_000_007; // Don't forget this...

    s.chars()
    .map(|c| -> bool { c == '1' })
    .fold(
        (0, 0),
        |(prev_group_len, ans_num), curr_one_flag| -> (i32, i32) {
            if curr_one_flag {
                ((prev_group_len + 1) % MODULO, (ans_num + prev_group_len + 1) % MODULO)
            } else {
                (0, ans_num)
            }
        },
    )
    .1
}