//! # Leetcode 2273. Find Resultant Array After Removing Anagrams
//! https://leetcode.com/problems/find-resultant-array-after-removing-anagrams/
//! - `Easy`; `y2025m10d13`; `Independently Solved`; `0ms`; `2.3mb`; `1 attempt`;
//! Topics: count_tracking.

pub fn remove_anagrams(words: Vec<String>) -> Vec<String> {
            const LOWER_LETTER_NUM: usize = 26;
    fn write_word_letter_count_arr_to_target(
        word: &str,
        ans_count_arr_mut_ref: &mut [usize; LOWER_LETTER_NUM],
    ) {
        for entry_mut_ref in ans_count_arr_mut_ref.iter_mut() {
            *entry_mut_ref = 0;
        }
        for c_u8 in word.as_bytes().iter().cloned() {
            ans_count_arr_mut_ref[(c_u8 - b'a') as usize] += 1;
        }
    }

    let mut ans_string_vec = Vec::with_capacity(words.len());
    let mut word_iter = words.into_iter();

    let first_word = word_iter.next().expect("len > 0");

    let mut prev_count_arr: [usize; LOWER_LETTER_NUM] = [0; LOWER_LETTER_NUM];
    let mut curr_count_arr: [usize; LOWER_LETTER_NUM] = [0; LOWER_LETTER_NUM];

    write_word_letter_count_arr_to_target(&first_word, &mut prev_count_arr);

    ans_string_vec.push(first_word);

    for word in word_iter {
        write_word_letter_count_arr_to_target(&word, &mut curr_count_arr);

        if curr_count_arr == prev_count_arr {
            continue;
        }

        ans_string_vec.push(word);

        std::mem::swap(&mut prev_count_arr, &mut curr_count_arr);
    }

    ans_string_vec
}