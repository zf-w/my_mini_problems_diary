//! # Make String a Subsequence Using Cyclic Increments
//! https://leetcode.com/problems/make-string-a-subsequence-using-cyclic-increments/
//! - `Medium`; `y2024m12d04`; `Hinted 2`; `3ms`; `2.7m`; `1 attempt`;

pub fn can_make_subsequence(str1: String, str2: String) -> bool {
    let mut char_iter_1 = str1.chars();
    let mut char_iter_2 = str2.chars();

    fn shift_c(c: char) -> char {
        if ('b'..='z').contains(&c) {
            (c as u8 - 1) as char
        } else {
            'z'
        }
    }

    let mut pending_0_c = char_iter_2.next().expect("At least 1");
    let mut pending_1_c = shift_c(pending_0_c);
    for c_1 in char_iter_1 {
        if c_1 == pending_0_c || c_1 == pending_1_c {
            if let Some(next_c) = char_iter_2.next() {
                pending_0_c = next_c;
                pending_1_c = shift_c(next_c);
            } else {
                return true;
            }
        }
    }
    false
}
