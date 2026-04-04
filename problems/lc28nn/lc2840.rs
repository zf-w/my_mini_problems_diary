//! # LeetCode 2840. Check if Strings Can be Made Equal With Operations II
//! https://leetcode.com/problems/check-if-strings-can-be-made-equal-with-operations-ii/
//! - y2026m03d30; Independently Solved;

pub fn check_strings(s1: String, s2: String) -> bool {
    const LOWER_LETTER_COUNT: usize = 26;
    let mut even_c_count_arr: [i32; LOWER_LETTER_COUNT] = [0; LOWER_LETTER_COUNT];
    let mut odd_c_count_arr: [i32; LOWER_LETTER_COUNT] = [0; LOWER_LETTER_COUNT];

    let s1_u8_arr_ref = s1.as_bytes();
    let s2_u8_arr_ref = s2.as_bytes();

    fn c_u8_to_c_idx(c_u8: u8) -> usize {
        (c_u8 - b'a') as usize
    }

    for (i, (s1_u8, s2_u8)) in s1_u8_arr_ref
        .iter()
        .cloned()
        .zip(s2_u8_arr_ref.iter().cloned())
        .enumerate()
    {
        let s1_idx = c_u8_to_c_idx(s1_u8);
        let s2_idx = c_u8_to_c_idx(s2_u8);
        if i & 1 == 0 {
            even_c_count_arr[s1_idx] += 1;
            even_c_count_arr[s2_idx] -= 1;
        } else {
            odd_c_count_arr[s1_idx] += 1;
            odd_c_count_arr[s2_idx] -= 1;
        }
    }

    even_c_count_arr
        .into_iter()
        .chain(odd_c_count_arr)
        .fold(true, |prev_flag, c_count: i32| -> bool {
            prev_flag & (c_count == 0)
        })
}