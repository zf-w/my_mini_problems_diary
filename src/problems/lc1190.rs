//! ## Leetcode 1190. Reverse Substrings Between Each Pair of Parentheses
//! https://leetcode.com/problems/reverse-substrings-between-each-pair-of-parentheses/
//! - `Medium`; `Independently Solved`; `2024-07-11`;

pub fn reverse_parentheses(s: String) -> String {
    let mut chars: Vec<u8> = Vec::with_capacity(s.len());
    let mut bars: Vec<usize> = Vec::new();
    for c in s.chars() {
        if c == '(' {
            bars.push(chars.len());
        } else if c == ')' {
            let prev_i = bars.pop().expect("Balanced parentheses");
            let end_i = chars.len();
            let to_reverse = &mut chars[prev_i..end_i];
            to_reverse.reverse();
        } else {
            chars.push(c as u8);
        }
    }
    String::from_utf8(chars).expect("Should be valid")
}
