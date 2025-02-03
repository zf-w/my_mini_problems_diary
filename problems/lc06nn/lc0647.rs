//! ## Leetcode 647. Palindromic Substrings
//! https://leetcode.com/problems/palindromic-substrings
//! - `Medium`; `Independently Solved`; `2024-02-10`;
//!
//! For Strings in Rust, if given that the characters are English letters, viewing the string as an `&[u8]` might be a better choice.

pub fn count_substrings(s: String) -> i32 {
    let char_arr = s.as_bytes();
    let mut ans = 0;
    let len = s.len();
    for i in 0..len {
        ans += 1;
        let mut prev = i;
        let mut next = i;

        while prev > 0 && next < len - 1 {
            prev -= 1;
            next += 1;
            if char_arr[prev] == char_arr[next] {
                ans += 1;
            } else {
                break;
            }
        }
    }
    for i in 1..len {
        let mut prev = i;
        let mut next = i - 1;

        while prev > 0 && next < len - 1 {
            prev -= 1;
            next += 1;
            if char_arr[prev] == char_arr[next] {
                ans += 1;
            } else {
                break;
            }
        }
    }
    ans
}
