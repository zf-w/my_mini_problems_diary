//! ## Leetcode 3081. Replace Question Marks in String to Minimize Its Value
//! https://leetcode.com/problems/replace-question-marks-in-string-to-minimize-its-value/
//! - `Medium`; `Learned from Solution`; `2024-03-16`;
//!
//! This question is the third question of Leetcode Biweekly Contest 126.
//!
//! I figured out to count the freqency of occured characters, but I'm not sure why I can use sorting to get the optimal result.

pub fn minimize_string_value(s: String) -> String {
    let mut counts: [usize; 26] = [0; 26];
    let base = 'a' as usize;
    let map_char = |c: char| -> usize { c as usize - base };
    let s_len = s.len();
    let mut ans: String = String::with_capacity(s_len);
    let mut char_arr: Vec<char> = Vec::with_capacity(s_len);
    for c in s.chars() {
        if c != '?' {
            counts[map_char(c)] += 1;
        }
    }
    for c in s.chars() {
        if c == '?' {
            let mut min_i = 0;
            let mut min_count = counts[0];
            for (i, count) in counts.iter().enumerate() {
                if count < &min_count {
                    min_count = *count;
                    min_i = i;
                }
            }
            counts[min_i] += 1;
            char_arr.push((base + min_i) as u8 as char);
        }
    }
    char_arr.sort();
    let mut i = 0;
    for c in s.chars() {
        if c == '?' {
            ans.push(char_arr[i]);
            i += 1;
        } else {
            ans.push(c);
        }
    }
    ans
}

#[test]
fn q3_case_1() {
    let s = "???".to_string();
    assert_eq!("abc".to_string(), minimize_string_value(s));
}

#[test]
fn q3_case_2() {
    let s = "a?a?".to_string();
    assert_eq!("abac".to_string(), minimize_string_value(s));
}
