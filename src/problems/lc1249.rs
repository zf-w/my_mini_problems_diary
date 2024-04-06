//! ## Leetcode 1249. Minimum Remove to Make Valid Parentheses
//! https://leetcode.com/problems/minimum-remove-to-make-valid-parentheses
//! - `Medium`; `Independently Solved`; `2024-04-05`;
//!
//! Marking the not matching and left out brackets and removing them in the second run.

pub fn min_remove_to_make_valid(s: String) -> String {
    let len = s.len();
    let mut st: Vec<usize> = Vec::with_capacity(len);
    let mut valid: Vec<bool> = vec![true; len];
    for (i, c) in s.char_indices() {
        match c {
            '(' => {
                st.push(i);
            }
            ')' => {
                if st.is_empty() {
                    valid[i] = false;
                } else {
                    st.pop();
                }
            }
            _ => (),
        }
    }
    for left_out_i in st {
        valid[left_out_i] = false;
    }
    let mut ans: String = String::with_capacity(len);
    for (i, c) in s.char_indices() {
        if valid[i] {
            ans.push(c);
        }
    }
    ans
}
