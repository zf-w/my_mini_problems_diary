//! # Leetcode 3306. Count of Substrings Containing Every Vowel and K Consonants II
//! https://leetcode.com/problems/count-of-substrings-containing-every-vowel-and-k-consonants-ii/
//! - `Medium`; `y2025m03d10`; `Learned from Solution`; `51ms`; `4.5mb`; `4 attempts`;
//!
//! Topics: sliding_window.
//!
//! I didn't think in the direction of "substrings expanding from the 'end' of the window." I was focused and stuck on the beginning side of the window, hahaha. The "end-window" solution is pretty elegant.

pub fn count_of_substrings(word: String, k: i32) -> i64 {
    let k = k as usize;
    let word_len = word.len();
    fn vowel_char_u8_to_idx(c_u8_ref: &u8) -> usize {
        match *c_u8_ref {
            b'a' => 0,
            b'e' => 1,
            b'i' => 2,
            b'o' => 3,
            b'u' => 4,
            _ => 5,
        }
    }

    let word_byte_arr_ref = word.as_bytes();
    let mut next_consonant_i = word_len;
    let mut next_consonant_vec: Vec<usize> = vec![0; word_len];

    for i in (0..word_len).rev() {
        next_consonant_vec[i] = next_consonant_i;
        if vowel_char_u8_to_idx(&word_byte_arr_ref[i]) == VOWEL_NUM {
            next_consonant_i = i;
        }
    }

    let mut begin_iter = word_byte_arr_ref.iter().map(vowel_char_u8_to_idx);

    let end_iter = word_byte_arr_ref
        .iter()
        .map(vowel_char_u8_to_idx)
        .enumerate();

    const VOWEL_NUM: usize = 5;

    let mut consonant_num: usize = 0;
    let mut vowel_num_arr: [usize; 5] = [0; VOWEL_NUM];

    let mut unique_vowel_num: usize = 0;

    let mut ans_num: usize = 0;

    for (end_i, end_char_idx) in end_iter {
        if end_char_idx < VOWEL_NUM {
            let vowel_arr_entry_mut_ref = &mut vowel_num_arr[end_char_idx];
            if *vowel_arr_entry_mut_ref == 0 {
                unique_vowel_num += 1;
            }
            *vowel_arr_entry_mut_ref += 1;
        } else {
            consonant_num += 1;
        }

        while consonant_num > k {
            let begin_char_idx = begin_iter.next().expect("begin <= end");

            if begin_char_idx < VOWEL_NUM {
                let vowel_arr_entry_mut_ref = &mut vowel_num_arr[begin_char_idx];
                *vowel_arr_entry_mut_ref -= 1;
                if *vowel_arr_entry_mut_ref == 0 {
                    unique_vowel_num -= 1;
                }
            } else {
                consonant_num -= 1;
            }
        }

        let add_num = next_consonant_vec[end_i] - end_i;

        while consonant_num == k && unique_vowel_num == VOWEL_NUM {
            let begin_char_idx = begin_iter.next().expect("begin <= end");

            ans_num += add_num;
            if begin_char_idx < VOWEL_NUM {
                let vowel_arr_entry_mut_ref = &mut vowel_num_arr[begin_char_idx];
                *vowel_arr_entry_mut_ref -= 1;
                if *vowel_arr_entry_mut_ref == 0 {
                    unique_vowel_num -= 1;
                }
            } else {
                consonant_num -= 1;
            }
        }
    }

    ans_num as i64
}
