//! # Leetcode 3403. Find the Lexicographically Largest String From the Box I
//! https://leetcode.com/problems/find-the-lexicographically-largest-string-from-the-box-i/
//! - `Medium`; `y2025m06d04`; `Independently Solved`; `15ms`; `2.3mb`; `5 attempts`;
//! Topics: uncategorized.

pub fn answer_string(word: String, num_friends: i32) -> String {
    if num_friends == 1 {
        return word;
    }

    let word_byte_arr_ref = word.as_bytes();

    let word_len = word_byte_arr_ref.len();

    let friend_num = num_friends as usize;

    fn tell_if_need_replace(word_0_arr_ref: &[u8], word_1_arr_ref: &[u8]) -> bool {
        for (word_1_byte_ref, word_0_byte_ref) in word_1_arr_ref.iter().zip(word_0_arr_ref.iter()) {
            if *word_1_byte_ref > *word_0_byte_ref {
                return true;
            } else if *word_1_byte_ref < *word_0_byte_ref {
                return false;
            }
        }

        word_1_arr_ref.len() > word_0_arr_ref.len()
    }

    let pick_word_len = word_len - friend_num + 1;

    let mut ans_byte_arr_ref: &[u8] = &word_byte_arr_ref[..pick_word_len];

    for begin_i in 0..word_len {
        let end_i = (begin_i + pick_word_len).min(word_len);
        if tell_if_need_replace(ans_byte_arr_ref, &word_byte_arr_ref[begin_i..end_i]) {
            ans_byte_arr_ref = &word_byte_arr_ref[begin_i..end_i];
        }
    }

    unsafe { String::from_utf8_unchecked(ans_byte_arr_ref.to_vec()) }
}
