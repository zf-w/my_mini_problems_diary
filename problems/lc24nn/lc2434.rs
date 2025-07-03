//! # Leetcode 2434. Using a Robot to Print the Lexicographically Smallest String
//! https://leetcode.com/problems/using-a-robot-to-print-the-lexicographically-smallest-string/
//! - `Medium`; `y2025m06d06`; `Hinted`; `13ms`; `2.5mb`; `1 attempt`'
//! Topic: greedy.

pub fn robot_with_string(s: String) -> String {
    let s_char_arr_ref = s.as_bytes();
    let len = s_char_arr_ref.len();

    fn lower_char_ref_to_idx(c_ref: &u8) -> usize {
        (*c_ref - b'a') as usize
    }

    fn idx_to_lower_char(idx: usize) -> char {
        (b'a' + idx as u8) as char
    }

    let mut letter_count_arr: [usize; 26] = [0; 26];

    let mut min_idx: usize = 26;

    for idx in s_char_arr_ref.iter().map(lower_char_ref_to_idx) {
        letter_count_arr[idx] += 1;
        min_idx = min_idx.min(idx);
    }

    let mut stack: Vec<u8> = Vec::with_capacity(len);

    let mut ans_string: String = String::with_capacity(len);

    fn get_min_char(
        letter_count_arr_mut_ref: &mut [usize; 26],
        min_idx_mut_ref: &mut usize,
    ) -> Option<u8> {
        if *min_idx_mut_ref > 25 {
            return None;
        }

        while *min_idx_mut_ref < 26 && letter_count_arr_mut_ref[*min_idx_mut_ref] == 0 {
            *min_idx_mut_ref += 1;
        }

        if *min_idx_mut_ref > 25 {
            return None;
        }

        Some(*min_idx_mut_ref as u8 + b'a')
    }

    let mut i: usize = 0;

    while i < len {
        match (
            stack.last().cloned(),
            get_min_char(&mut letter_count_arr, &mut min_idx),
        ) {
            (None, None) => break,
            (None, Some(_)) => {
                let to_push_char_u8 = s_char_arr_ref[i];
                stack.push(to_push_char_u8);
                i += 1;
                letter_count_arr[lower_char_ref_to_idx(&to_push_char_u8)] -= 1;
            }
            (Some(_), None) => {
                ans_string.push(stack.pop().expect("Checked") as char);
            }
            (Some(char_0_u8), Some(char_1_u8)) => {
                if char_0_u8 <= char_1_u8 {
                    ans_string.push(stack.pop().expect("Checked") as char);
                } else {
                    let to_push_char_u8 = s_char_arr_ref[i];
                    stack.push(to_push_char_u8);
                    i += 1;
                    letter_count_arr[lower_char_ref_to_idx(&to_push_char_u8)] -= 1;
                }
            }
        }
    }

    while let Some(c_u8) = stack.pop() {
        ans_string.push(c_u8 as char);
    }

    ans_string
}
