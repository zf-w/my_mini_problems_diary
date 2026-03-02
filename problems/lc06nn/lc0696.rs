//! # Leetcode 696. Count Binary Substrings
//! https://leetcode.com/problems/count-binary-substrings/
//! - Easy; Independently Solved;


pub fn count_binary_substrings(s: String) -> i32 {
    s.as_bytes()
        .iter()
        .cloned()
        .fold(
            (true, 0, 0, 0),
            |(prev_bit_flag, prev_count, curr_count, ans_count), c_u8| -> (bool, i32, i32, i32) {
                if c_u8 == b'0' {
                    if prev_bit_flag == false {
                        let add = if prev_count >= (curr_count + 1) { 1 } else { 0 };
                        (false, prev_count, curr_count + 1, ans_count + add)
                    } else {
                        let add = if curr_count >= 1 { 1 } else { 0 };
                        (false, curr_count, 1, ans_count + add)
                    }
                } else {
                    if prev_bit_flag == true {
                        let add = if prev_count >= (curr_count + 1) { 1 } else { 0 };
                        (true, prev_count, curr_count + 1, ans_count + add)
                    } else {
                        let add = if curr_count >= 1 { 1 } else { 0 };
                        (true, curr_count, 1, ans_count + add)
                    }
                }
            },
        )
        .3
}
