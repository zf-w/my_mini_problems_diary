//! ## Leetcode 678. Valid Parenthesis String
//! https://leetcode.com/problems/valid-parenthesis-string
//! - `Medium`; `Learned from Solution`; `2024-04-07`;
//!
//! Intuitively, I figured out that I could try to use normal left brackets and then stars to pair the right ones. Then, I can try to match the left-out brackets with the remaining stars. Not sure why this works.

pub fn check_valid_string(s: String) -> bool {
    let len: usize = s.len();
    let mut star_st: Vec<usize> = Vec::with_capacity(len);
    let mut left_st: Vec<usize> = Vec::with_capacity(len);
    for (i, c) in s.char_indices() {
        match c {
            '(' => left_st.push(i),
            ')' => {
                if left_st.len() > 0 {
                    left_st.pop();
                } else if star_st.len() > 0 {
                    star_st.pop();
                } else {
                    // println!("1");
                    return false;
                }
            }
            '*' => {
                star_st.push(i);
            }
            _ => (),
        }
    }
    while let Some(left_i) = left_st.pop() {
        if let Some(last_i) = star_st.pop() {
            if last_i < left_i {
                // println!("2");
                return false;
            }
        } else {
            return false;
        }
    }
    true
}
