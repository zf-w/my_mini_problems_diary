//! # Leetcode 3170. Lexicographically Minimum String After Removing Stars
//! https://leetcode.com/problems/lexicographically-minimum-string-after-removing-stars/
//! - `Medium`; `y2025m06d07`; `Learned from Solution`; `21ms`; `3.3mb`; `1 attempt`;
//! Topics: stack;
//!
//! I haven't thought of such an expensive data structure haha.

pub fn clear_stars(mut s: String) -> String {
    const CHAR_LOWER_LETTER_NUM: usize = 26;
    const CHAR_LOWER_A_U8: u8 = b'a';
    const CHAR_ASTERISK: u8 = b'*';

    let len = s.len();
    let s_char_u8_arr_mut_ref = unsafe { s.as_bytes_mut() };
    let mut char_idx_stack_arr: [Vec<usize>; CHAR_LOWER_LETTER_NUM] =
        core::array::from_fn(|_| -> Vec<usize> { Vec::with_capacity(len) });

    fn char_u8_to_idx(c_u8: u8) -> usize {
        (c_u8 - CHAR_LOWER_A_U8) as usize
    }

    let mut min_char_idx: usize = CHAR_LOWER_LETTER_NUM;

    for i in 0..len {
        let c_u8 = s_char_u8_arr_mut_ref[i];
        let c_idx = char_u8_to_idx(c_u8);
        if c_u8 != CHAR_ASTERISK {
            char_idx_stack_arr[c_idx].push(i);
            min_char_idx = min_char_idx.min(c_idx);
            continue;
        }

        if min_char_idx == CHAR_LOWER_LETTER_NUM {
            continue;
        }

        let remove_idx = char_idx_stack_arr[min_char_idx].pop().expect("Should have");

        s_char_u8_arr_mut_ref[remove_idx] = CHAR_ASTERISK;

        while min_char_idx < CHAR_LOWER_LETTER_NUM && char_idx_stack_arr[min_char_idx].is_empty() {
            min_char_idx += 1;
        }
    }

    let mut write_i: usize = 0;
    let mut read_i: usize = 0;

    let mut ans_len = len;

    while read_i < len {
        if s_char_u8_arr_mut_ref[read_i] == CHAR_ASTERISK {
            read_i += 1;
            ans_len -= 1;
            continue;
        }

        if read_i > write_i {
            s_char_u8_arr_mut_ref[write_i] = s_char_u8_arr_mut_ref[read_i];
        }

        write_i += 1;
        read_i += 1;
    }

    s.truncate(ans_len);

    s
}
