//! # Leetcode 1092. Shortest Common Supersequence
//! https://leetcode.com/problems/shortest-common-supersequence/
//! - `Hard`; `y2025m02d28`; `Hinted`; `10ms`; `18.1mb`; `1 attempt`;
//!
//! Topics: dynamic_programming.
//!
//! Watch out for the out-of-range bugs. Hahaha.

pub fn shortest_common_supersequence(str1: String, str2: String) -> String {
    let str1_byte_arr_ref = str1.as_bytes();
    let str2_byte_arr_ref = str2.as_bytes();
    let str1_len = str1_byte_arr_ref.len();
    let str2_len = str2_byte_arr_ref.len();

    let str1_dp_len = str1_len + 1;
    let str2_dp_len = str2_len + 1;

    let mut dp_arr_ref: Box<[(usize, u8)]> =
        vec![(0, 0); str2_dp_len * str1_dp_len].into_boxed_slice();

    let calc_index =
        |str1_dp_i: usize, str2_dp_i: usize| -> usize { str1_dp_i * str2_dp_len + str2_dp_i };

    for str1_dp_i in 1..=str1_len {
        dp_arr_ref[calc_index(str1_dp_i, 0)] = (0, 2);
    }

    for str2_dp_i in 1..=str2_len {
        dp_arr_ref[calc_index(0, str2_dp_i)] = (0, 1);
    }

    for str1_dp_i in 1..=str1_len {
        let str1_char_i = str1_dp_i - 1;
        for str2_dp_i in 1..=str2_len {
            let str2_char_i = str2_dp_i - 1;
            if str1_byte_arr_ref[str1_char_i] == str2_byte_arr_ref[str2_char_i] {
                let next_len = dp_arr_ref[calc_index(str1_dp_i - 1, str2_dp_i - 1)].0 + 1;
                dp_arr_ref[calc_index(str1_dp_i, str2_dp_i)] = (next_len, 3);
            } else {
                let row_prev = dp_arr_ref[calc_index(str1_dp_i, str2_dp_i - 1)].0;
                let col_prev = dp_arr_ref[calc_index(str1_dp_i - 1, str2_dp_i)].0;
                dp_arr_ref[calc_index(str1_dp_i, str2_dp_i)] = if row_prev > col_prev {
                    (row_prev, 1)
                } else {
                    (col_prev, 2)
                }
            }
        }
    }

    let mut dp_row_i = str1_len;
    let mut dp_col_i = str2_len;

    let mut ans_char_u8_vec = Vec::with_capacity(str1_len + str2_len);

    while (dp_row_i > 0 || dp_col_i > 0) {
        let direction = dp_arr_ref[calc_index(dp_row_i, dp_col_i)].1;
        if direction == 3 {
            ans_char_u8_vec.push(str1_byte_arr_ref[dp_row_i - 1]);
            dp_row_i -= 1;
            dp_col_i -= 1;
        } else if direction == 2 {
            ans_char_u8_vec.push(str1_byte_arr_ref[dp_row_i - 1]);
            dp_row_i -= 1;
        } else {
            ans_char_u8_vec.push(str2_byte_arr_ref[dp_col_i - 1]);
            dp_col_i -= 1;
        }
    }
    ans_char_u8_vec.reverse();
    String::from_utf8(ans_char_u8_vec).expect("Should be valid.")
}
