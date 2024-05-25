//! ## Leetcode 140. Work Break II
//! https://leetcode.com/problems/word-break-ii
//! - `Hard`; `Independently Solved`; `2024-05-25`;
//!
//! I guess, due to the small problem size, plain recursion with backtracking is still working. For the mapping design, I guess a prefix tree might be more efficient, considering the early quit.

use std::collections::HashMap;

fn build_ans_string_from_path(path: &[usize], words: &[String]) -> String {
    let mut len: usize = 0;
    for word_i in path.iter() {
        len += words[*word_i].len();
    }
    len += path.len() - 1; // No 0 length input s...
    let mut ans_string = String::with_capacity(len);
    let mut path_iter = path.iter();
    if let Some(first_i) = path_iter.next() {
        ans_string.push_str(&words[*first_i]);
        while let Some(word_i) = path_iter.next() {
            ans_string.push(' ');
            ans_string.push_str(&words[*word_i]);
        }
    }
    ans_string
}

fn helper(
    s: &str,
    word_map: &HashMap<String, usize>,
    words: &[String],
    i_path: &mut Vec<usize>,
    ans_strings: &mut Vec<String>,
) {
    for i in 0..=s.len() {
        let curr_part = &s[0..i];
        if let Some(word_i) = word_map.get(curr_part) {
            i_path.push(*word_i);
            if i < s.len() {
                helper(&s[i..], word_map, words, i_path, ans_strings);
            } else {
                ans_strings.push(build_ans_string_from_path(i_path, words))
            }
            i_path.pop();
        }
    }
}

pub fn word_break(s: String, word_dict: Vec<String>) -> Vec<String> {
    let mut ans_strings: Vec<String> = Vec::new();
    let mut word_map: HashMap<String, usize> = HashMap::with_capacity(word_dict.len());
    for (i, word) in word_dict.iter().enumerate() {
        word_map.insert(word.clone(), i);
    }
    let mut i_path: Vec<usize> = Vec::with_capacity(word_dict.len());
    helper(&s, &word_map, &word_dict, &mut i_path, &mut ans_strings);
    ans_strings
}
