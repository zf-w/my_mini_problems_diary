//! # Leetcode 3163. String Compression III
//! https://leetcode.com/problems/string-compression-iii/
//! - `Medium`; `y2024m11d04`; `Independently Solved`; `7ms`; `2.8mb`; `1 attempt`;

pub fn compressed_string(word: String) -> String {
    let mut char_iter = word.chars();
    let mut prev_c = char_iter.next().expect("len >= 1");
    let mut prev_count: usize = 1;
    let mut ans_string = String::with_capacity(word.len() * 2);

    fn one_digit_usize_to_c(d: usize) -> char {
        const CHAR_0_U8: u8 = b'0';
        (CHAR_0_U8 + d as u8) as char
    }

    for c in char_iter {
        if c != prev_c {
            ans_string.push(one_digit_usize_to_c(prev_count));
            ans_string.push(prev_c);
            prev_c = c;
            prev_count = 1;
        } else if prev_count == 9 {
            ans_string.push(one_digit_usize_to_c(prev_count));
            ans_string.push(prev_c);
            prev_count = 1;
        } else {
            prev_count += 1;
        }
    }
    if prev_count > 0 {
        ans_string.push(one_digit_usize_to_c(prev_count));
        ans_string.push(prev_c);
    }
    ans_string
}
