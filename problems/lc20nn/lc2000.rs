//! ## Leetcode 2000. Reverse Prefix of Word
//! https://leetcode.com/problems/reverse-prefix-of-word
//! - `Easy`; `Independently Solved`; `2024-04-30`;
//!
//! A question about reversing the starting part of a string.

pub fn reverse_prefix(mut word: String, ch: char) -> String {
    let len = word.len();
    let word_arr = unsafe { word.as_bytes_mut() };
    let ch_u8 = ch as u8;
    let mut ch_i = 0;

    while ch_i < len && word_arr[ch_i] != ch_u8 {
        ch_i += 1;
    }
    let mut begin_i = 0usize;
    while begin_i < ch_i {
        let temp = word_arr[begin_i];
        word_arr[begin_i] = word_arr[ch_i];
        word_arr[ch_i] = temp;
        begin_i += 1;
        ch_i -= 1;
    }
    word
}
