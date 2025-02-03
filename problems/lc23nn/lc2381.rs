//! # Leetcode 2381. Shifting Letters II
//! https://leetcode.com/problems/shifting-letters-ii/
//! - `Medium`; `y2025m01d05`; `Hinted`; `3ms`; `6.1mb`; `1 attempt`;

pub fn shifting_letters(s: String, shifts: Vec<Vec<i32>>) -> String {
    let s_bytes_ref = s.as_bytes();
    let len = s_bytes_ref.len();
    let mut sum_vec: Vec<i32> = vec![0; len + 1];
    const LETTER_NUM: i32 = 26;
    const CHAR_LOWER_A_U8: u8 = b'a';
    for shift_vec in shifts {
        let begin_i = shift_vec[0] as usize;
        let end_i = shift_vec[1] as usize + 1;
        let shift_val = if shift_vec[2] == 1 { 1 } else { -1 };
        sum_vec[begin_i] = (sum_vec[begin_i] + LETTER_NUM + shift_val) % LETTER_NUM;
        sum_vec[end_i] = (sum_vec[end_i] + LETTER_NUM - shift_val) % LETTER_NUM;
    }
    let mut ans_char_bytes_vec: Vec<u8> = Vec::with_capacity(len);
    let mut shift_sum: i32 = 0;
    for (byte, shift_val) in s_bytes_ref.iter().cloned().zip(sum_vec.into_iter()) {
        shift_sum += shift_val;
        ans_char_bytes_vec.push(
            ((((byte - CHAR_LOWER_A_U8) as i32) + shift_sum) % LETTER_NUM) as u8 + CHAR_LOWER_A_U8,
        );
    }
    String::from_utf8(ans_char_bytes_vec).expect("expecting valid utf-8.")
}
