//! # Leetcode 1358. Number of Substrings Containing All Three Characters
//! https://leetcode.com/problems/number-of-substrings-containing-all-three-characters/
//! - `Medium`; `y2025m03d11`; `Independently Solved`; `1ms`; `2.3mb`; `1 attempt`;
//! Topics: sliding_window.

pub fn number_of_substrings(s: String) -> i32 {
    let s_byte_arr_ref = s.as_bytes();

    fn map_key_char_to_idx(c_ref: &u8) -> usize {
        (*c_ref - b'a') as usize
    }

    let len = s_byte_arr_ref.len();

    let mut begin_iter = s_byte_arr_ref.iter().map(map_key_char_to_idx);
    let end_iter = s_byte_arr_ref.iter().map(map_key_char_to_idx).enumerate();

    const KEY_CHAR_NUM: usize = 3;
    let mut key_char_count_arr: [usize; 3] = [0; KEY_CHAR_NUM];

    let mut key_char_count: usize = 0;

    let mut ans_num = 0;

    for (end_i, end_char_idx) in end_iter {
        let end_entry_mut_ref = &mut key_char_count_arr[end_char_idx];

        if *end_entry_mut_ref == 0 {
            key_char_count += 1;
        }

        *end_entry_mut_ref += 1;

        while key_char_count == 3 {
            ans_num += len - end_i;

            let begin_char_idx = begin_iter.next().expect("begin <= end");

            let begin_entry_mut_ref = &mut key_char_count_arr[begin_char_idx];

            *begin_entry_mut_ref -= 1;

            if *begin_entry_mut_ref == 0 {
                key_char_count -= 1;
            }

            ans_num += end_char_idx
        }
    }
    ans_num as i32
}
