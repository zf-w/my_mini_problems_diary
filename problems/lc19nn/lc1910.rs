//! # Leetcode 1910. Remove All Occurrences of a Substring
//! https://leetcode.com/problems/remove-all-occurrences-of-a-substring/
//! - `Medium`; `y2025m02d10`; `Independently Solved`; `0ms`; `2.3mb`; `2 attempts`;
//!
//! Topics: "stack"

pub fn remove_occurrences(s: String, part: String) -> String {
    let part_len = part.len();
    let part_str_byte_arr_ref: &[u8] = part.as_bytes();
    let mut ans_string_byte_vec: Vec<u8> = Vec::with_capacity(s.len());

    let mut ans_len: usize = 0;

    for s_c_u8 in s.as_bytes().iter().cloned() {
        ans_string_byte_vec.push(s_c_u8);
        ans_len += 1;
        // First bug: forgot the equal sign below.
        if ans_len >= part_len {
            let new_len = ans_len - part_len;
            if ans_string_byte_vec[new_len..ans_len].eq(part_str_byte_arr_ref) {
                ans_string_byte_vec.resize(new_len, 0);

                ans_len = new_len;
            }
        }
        // println!("{}", String::from_utf8(ans_string_byte_vec.clone()).expect("Should be valid utf-8"));
    }

    String::from_utf8(ans_string_byte_vec).expect("Should be valid utf-8")
}
