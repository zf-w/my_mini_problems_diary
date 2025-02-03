//! ## Leetcode 1002. Find Common Characters
//! https://leetcode.com/problems/find-common-characters/
//! - `Easy`; `Independently Solved`; `2024-06-06`;
//!
//! Looping through the words, we can keep track of the minimum value of each character's number of occurrences in one word.

const BASE_LOWER_A: u8 = 'a' as u8;

pub fn common_chars(words: Vec<String>) -> Vec<String> {
    let mut all_count: [usize; 26] = [0; 26];

    fn char_to_i(c: char) -> usize {
        (c as u8 - BASE_LOWER_A) as usize
    }
    fn i_to_char(i: usize) -> char {
        (BASE_LOWER_A + i as u8) as char
    }
    let mut one_word_count: [usize; 26] = [0; 26];
    let mut words_iter = words.iter();
    if let Some(word) = words_iter.next() {
        for c in word.chars() {
            all_count[char_to_i(c)] += 1;
        }
    }

    for word in words_iter {
        for c in word.chars() {
            one_word_count[char_to_i(c)] += 1;
        }
        for (i, entry) in one_word_count.iter_mut().enumerate() {
            if *entry < all_count[i] {
                all_count[i] = *entry;
            }
            *entry = 0;
        }
    }
    let mut len = 0;
    for entry in all_count.iter() {
        len += *entry;
    }
    let mut ans_vec: Vec<String> = Vec::with_capacity(len);
    for (i, entry) in all_count.iter().enumerate() {
        let curr_char = i_to_char(i).to_string();
        for _ in 0..*entry {
            ans_vec.push(curr_char.clone());
        }
    }
    ans_vec
}
