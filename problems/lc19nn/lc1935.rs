//! # Leetcode 1935. Maximum Number of Words You Can Type
//! https://leetcode.com/problems/maximum-number-of-words-you-can-type/
//! - `Easy`; `y2025m09d15`; `Independently Solved`; `0ms`; `2.2mb`; `1 attempt`;
//! Topics: uncategorized.

pub fn can_be_typed_words(text: String, broken_letters: String) -> i32 {
    fn char_u8_to_idx(c_u8: u8) -> usize {
        (c_u8 - b'a') as usize
    }
    const LOWER_LETTER_NUM: usize = 26;
    let mut broken_letter_flag_arr: [bool; LOWER_LETTER_NUM] = [false; LOWER_LETTER_NUM];

    for c_u8_ref in broken_letters.as_bytes() {
        broken_letter_flag_arr[char_u8_to_idx(*c_u8_ref)] = true;
    }

    let mut word_flag = true;

    let mut ans_num = 0;

    for c_u8_ref in text.as_bytes() {
        if *c_u8_ref == b' ' {
            if word_flag {
                ans_num += 1;
            }
            word_flag = true;
        }

        if broken_letter_flag_arr[char_u8_to_idx(*c_u8_ref)] == true {
            word_flag = false;
        }
    }

    if word_flag {
        ans_num += 1;
    }

    ans_num
}
