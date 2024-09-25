//! ## Leetcode 1684. Count the Number of Consistent Strings
//! https://leetcode.com/problems/count-the-number-of-consistent-strings/
//! - `Easy`; `Independently Solved`; `2024-09-12`;

const BASE_A_U8: u8 = 'a' as u8;

pub fn count_consistent_strings(allowed: String, words: Vec<String>) -> i32 {
    let mut allow_char_idx_bool_arr: [bool; 26] = [false; 26];
    fn char_to_idx(c: char) -> usize {
        (c as u8 - BASE_A_U8) as usize
    }
    for allowed_c in allowed.chars() {
        allow_char_idx_bool_arr[char_to_idx(allowed_c)] = true;
    }
    let mut ans_count = 0;
    for word in words {
        let mut ok_flag = true;
        for c in word.chars() {
            if allow_char_idx_bool_arr[char_to_idx(c)] == false {
                ok_flag = false;
                break;
            }
        }
        if ok_flag {
            ans_count += 1;
        }
    }
    ans_count
}
