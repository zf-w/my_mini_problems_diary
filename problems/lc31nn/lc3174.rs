//! # Leetcode 3174. Clear Digits
//! https://leetcode.com/problems/clear-digits/
//! - `Easy`; `y2025m02d09`; `Independently Solved`; `0ms`; `2.26mb`; `1 attempt`;

pub fn clear_digits(s: String) -> String {
    const CHAR_0_U8: u8 = b'0';
    const CHAR_9_U8: u8 = b'9';
    const NUMERIC_RANGE: std::ops::RangeInclusive<u8> = CHAR_0_U8..=CHAR_9_U8;
    let mut ans_string_byte_vec: Vec<u8> = Vec::new();
    for char_u8 in s.as_bytes().iter().cloned() {
        if NUMERIC_RANGE.contains(&char_u8) {
            ans_string_byte_vec.pop();
        } else {
            ans_string_byte_vec.push(char_u8);
        }
    }
    String::from_utf8(ans_string_byte_vec).expect("Should be valid.")
}
