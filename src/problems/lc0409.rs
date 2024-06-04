//! ## Leetcode 409. Longest Palindrome
//! https://leetcode.com/problems/longest-palindrome/
//! - `Easy`; `Independently Solved`; `2024-06-04`;
//!
//! We can count the number of pairs of the identical characters with whether there is one odd extra character to put in the middle of the palindrome.

const BASE_UPPER_A: u8 = 'A' as u8;
const BASE_LOWER_A: u8 = 'a' as u8;

pub fn longest_palindrome(s: String) -> i32 {
    fn char_u8_to_i(c_u8: u8) -> usize {
        if c_u8 >= BASE_LOWER_A {
            // Missed one equal sign here
            (c_u8 - BASE_LOWER_A + 26) as usize
        } else {
            (c_u8 - BASE_UPPER_A) as usize
        }
    }
    let mut count: [bool; 52] = [false; 52];
    let mut odd_count: usize = 0;
    let mut even_pair_count: usize = 0;
    for c_u8_ref in s.as_bytes().iter() {
        let curr_count_mut_ref = &mut count[char_u8_to_i(*c_u8_ref)];

        if curr_count_mut_ref == &false {
            odd_count += 1;
            *curr_count_mut_ref = true;
        } else {
            odd_count -= 1;
            even_pair_count += 1;
            *curr_count_mut_ref = false;
        }
    }
    (2 * even_pair_count + if odd_count > 0 { 1 } else { 0 }) as i32
}
