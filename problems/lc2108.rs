//! ## Leetcode 2108. Find First Palindromic String in the Array
//! https://leetcode.com/problems/find-first-palindromic-string-in-the-array
//! - `Easy`; `Independently Solved`; `2024-02-13`;
//!
//! Just a normal problem about palindromes.

pub fn first_palindrome(words: Vec<String>) -> String {
    fn check(s: &str) -> bool {
        let arr: &[u8] = s.as_bytes();
        let begin: usize = 0;
        let end: usize = s.len() - 1;
        while begin < end {
            if arr[begin] != arr[end] {
                return false;
            }
        }
        true
    }
    for word in words {
        if check(&word) {
            return word;
        }
    }
    "".to_string()
}
