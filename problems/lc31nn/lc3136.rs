//! # Leetcode 3136. Valid Word
//! https://leetcode.com/problems/valid-word/
//! - `Easy`; `y2025m07d14`; `Independently Solved`; `0ms`; `2.1mb`; `1 attempt`;
//! Topics: uncategorized.

pub fn is_valid(word: String) -> bool {
    let word_byte_arr_ref = word.as_bytes();

    if word_byte_arr_ref.len() < 3 {
        return false;
    }

    let mut vowel_flag = false;
    let mut consonant_flag = false;

    for c in word_byte_arr_ref.iter().cloned() {
        if c.is_ascii_lowercase() || c.is_ascii_uppercase() {
            if c == b'A'
                || c == b'E'
                || c == b'I'
                || c == b'O'
                || c == b'U'
                || c == b'a'
                || c == b'e'
                || c == b'i'
                || c == b'o'
                || c == b'u'
            {
                vowel_flag = true;
            } else {
                consonant_flag = true;
            }
        } else if c.is_ascii_digit() == false {
            return false;
        }
    }
    vowel_flag && consonant_flag
}