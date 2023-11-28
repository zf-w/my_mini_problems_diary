//! ## Leetcode 1930. Unique Length-3 Palindromic Subsequence
//! 
//! https://leetcode.com/problems/unique-length-3-palindromic-subsequences
//! 
//! Given a string `s`, *return the number of **unique palindromes of length three** that are a **subsequence** of `s`*.
//! 
//! Note that even if there are multiple ways to obtain the same subsequence, it is still only counted `once`.
//! 
//! A `palindrome` is a string that reads the same forwards and backwards.
//! 
//! A `subsequence` of a string is a new string generated from the original string with some characters (can be none) deleted without changing the relative order of the remaining characters.
//! 
//! ### Example:
//! 
//! ```
//! use learn_cs::problems::lc1930;
//! let s: String = String::from("aabca");
//! 
//! assert_eq!(3, lc1930::count_palindromic_subsequence(s));
//! ``` 
//! 
//! ### Thoughts:
//! - `Independently Solved`; `2023-11-13`;
//! 
//! Since this problem only concerns palindromes of length `3`, when adding a new character to the end of the previous string, we only need to know whether there are substrings of length `2` starting with that character. My solution was fast!
//! 

pub fn count_palindromic_subsequence(s: String) -> i32 {
    let mut c1: [bool; 26] = [false; 26];
    let mut c2: [bool; 26 * 26] = [false; 26 * 26];
    let mut p3: [bool; 26 * 26] = [false; 26 * 26];
    fn idx(c: char) -> usize {
        (c as usize) - ('a' as usize)
    }
    let mut count: i32 = 0;
    for c in s.chars() {
        let curr = idx(c);
        let curr26 = curr * 26;
        for prev in 0..26 {
            if c2[curr26 + prev] && !p3[curr26 + prev] {
                p3[curr26 + prev] = true;
                count += 1;
            }
        }
        for prev in 0..26 {
            if c1[prev] {
                c2[prev * 26 + curr] = true;
            }
        }
        c1[curr] = true;
    }
    count
}