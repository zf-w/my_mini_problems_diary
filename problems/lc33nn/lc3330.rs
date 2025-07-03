//! # Leetcode 3330. Find the Original Typed String I
//! https://leetcode.com/problems/find-the-original-typed-string-i/
//! - `Easy`; `y2025m07d01`; `Independently Solved`; `2ms`; `2.1mb`; `1 attempt`;
//! Topics: uncategorized.

pub fn possible_string_count(word: String) -> i32 {
    let mut ans_count = 1;
    let mut byte_iter = word.as_bytes().iter().cloned();

    // let mut prev_char_count: usize = 0;
    let mut prev_char_u8: u8 = byte_iter.next().expect("len > 0");

    for c_u8 in byte_iter {
        if c_u8 == prev_char_u8 {
            ans_count += 1;
        } else {
            prev_char_u8 = c_u8;
        }
    }

    // if prev_char_count > 0 {
    //     ans_count += prev_char_count;
    // }

    ans_count as i32
}
