//! # LeetCode 2075. Decode the Slanted Ciphertext
//! https://leetcode.com/problems/decode-the-slanted-ciphertext/
//! - y2026m04d03; Independently Solved;

pub fn decode_ciphertext(encoded_text: String, rows: i32) -> String {
    let encoded_text_c_u8_ref = encoded_text.as_bytes();
    let len = encoded_text_c_u8_ref.len();
    let row_count = rows as usize;
    let col_count = len / row_count;

    let mut ans_string_vec = Vec::with_capacity(len);

    let mut idx = 0;
    let mut col_i: usize = 0;

    while idx < len {
        ans_string_vec.push(encoded_text_c_u8_ref[idx]);
        idx += col_count + 1;
        if idx == len {
            break;
        } else if idx > len {
            col_i += 1;
            idx = col_i;
        }
    }

    while let Some(c_u8) = ans_string_vec.last().cloned() {
        if c_u8 == b' ' {
            ans_string_vec.pop();
        } else {
            break;
        }
    }

    String::from_utf8(ans_string_vec).expect("should be valid...")
}
