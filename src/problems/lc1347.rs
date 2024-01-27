//! ## Leetcode 1347. Minimum Number of Steps to Make Two Strings Anagram
//! https://leetcode.com/problems/minimum-number-of-steps-to-make-two-strings-anagram
//! - `Medium`; `Independently Solved`; `2024-01-12`;
//!
//! Count the number of each character and sum up the number of missing characters. I guess the key to getting the minimum steps is to not change those that already have a match.

pub fn min_steps(s: String, t: String) -> i32 {
    let mut count: [i32; 26] = [0; 26];
    let base = 'a' as usize;
    for c in s.chars() {
        count[c as usize - base] += 1;
    }
    for c in t.chars() {
        count[c as usize - base] -= 1;
    }
    count.iter().filter(|x| x.is_positive()).sum::<i32>()
}
