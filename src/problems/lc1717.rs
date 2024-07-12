//! ## Leetcode 1717. Maximum Score From Removing Substrings
//! https://leetcode.com/problems/maximum-score-from-removing-substrings/
//! - `Medium`; `Hinted`; `2024-07-12`;

use std::char;

pub fn maximum_gain(s: String, x: i32, y: i32) -> i32 {
    let mut ans_score: i32 = 0;
    let mut chars_vec: Vec<char> = Vec::with_capacity(s.len());
    for curr_c in s.chars() {
        if let Some(last_c) = chars_vec.last().cloned() {
            if x > y && last_c == 'a' && curr_c == 'b' {
                chars_vec.pop();
                ans_score += x;
            } else if x < y && last_c == 'b' && curr_c == 'a' {
                chars_vec.pop();
                ans_score += y;
            } else {
                chars_vec.push(curr_c);
            }
        }
    }

    let mut chars_vec_1: Vec<char> = Vec::with_capacity(s.len());
    for curr_c in chars_vec.iter().cloned() {
        if let Some(last_c) = chars_vec.last().cloned() {
            if last_c == 'a' && curr_c == 'b' {
                chars_vec_1.pop();
                ans_score += x;
            } else if last_c == 'b' && curr_c == 'a' {
                chars_vec_1.pop();
                ans_score += y;
            } else {
                chars_vec_1.push(curr_c);
            }
        }
    }
    ans_score
}
