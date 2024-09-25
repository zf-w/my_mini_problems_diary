//! ## Leetcode 1653. Minimum Deletions to Make String Balanced
//! https://leetcode.com/problems/minimum-deletions-to-make-string-balanced/
//! - `Medium`; `Independently Solved`; `2024-07-30`;
//!
pub fn minimum_deletions(s: String) -> i32 {
    let mut prev_b_count: i32 = 0;
    let mut prev_ans: i32 = 0;
    for c in s.chars() {
        if c == 'a' {
            prev_ans = (prev_ans + 1).min(prev_b_count);
        } else {
            prev_b_count += 1;
        }
    }
    prev_ans
}
