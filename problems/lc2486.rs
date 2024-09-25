//! ## Leetcode 2486. Append Characters to String to Make Subsequence
//! https://leetcode.com/problems/append-characters-to-string-to-make-subsequence/
//! - `Medium`; `Independently Solved`; `2024-06-02`;
//!
//! We can solve this problem by iterating through string `s` and checking how many characters can we match with those in string `t` from the start.
//!

pub fn append_characters(s: String, t: String) -> i32 {
    let s_bytes = s.as_bytes();
    let t_bytes = t.as_bytes();
    let mut t_bytes_iter = t_bytes.iter();
    let mut wait_v = t_bytes_iter.next().expect("Longer than zero");
    for s_curr_byte in s_bytes.iter() {
        if s_curr_byte == wait_v {
            wait_v = if let Some(next_v) = t_bytes_iter.next() {
                next_v
            } else {
                return 0;
            }
        }
    }
    t_bytes_iter.len() as i32 + 1
}
