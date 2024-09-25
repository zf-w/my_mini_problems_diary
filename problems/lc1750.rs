//! ## Leetcode 1750. Minimum Length of String After Deleting Similar Ends
//! https://leetcode.com/problems/minimum-length-of-string-after-deleting-similar-ends
//! - `Medium`; `Independently Solved`; `2024-03-05`;
//!
//! The initialization of variables and while-loop conditions are indeed important.

pub fn minimum_length(s: String) -> i32 {
    let mut begin: usize = 0;
    let mut last: usize = s.len() - 1;

    let char_arr = s.as_bytes();

    while begin < last && char_arr[begin] == char_arr[last] {
        let curr_char = char_arr[begin];
        begin += 1;
        last -= 1;
        while begin <= last && char_arr[begin] == curr_char {
            begin += 1;
        }
        while begin <= last && char_arr[last] == curr_char {
            last -= 1;
        }
    }

    (last - begin + 1) as i32
}
