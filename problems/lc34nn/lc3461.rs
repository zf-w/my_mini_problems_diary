//! # Leetcode 3461. Check If Digits Are Equal in String After Operations
//! https://leetcode.com/problems/check-if-digits-are-equal-in-string-after-operations-i/
//! - `Easy`; `y2025m10d23`; `Independently Solved`; `0ms`; `2.3mb`; `1 attempt`;
//! Topics: simulation.

pub fn has_same_digits(mut s: String) -> bool {
    let char_arr_mut_ref = unsafe { s.as_bytes_mut() };

    for entry_mut_ref in char_arr_mut_ref.iter_mut() {
        *entry_mut_ref -= b'0';
    }

    let len = char_arr_mut_ref.len();

    for end in (2..len).rev() {
        for i in 1..end {
            char_arr_mut_ref[i - 1] = (char_arr_mut_ref[i - 1] + char_arr_mut_ref[i]) % 10;
        }
    }

    char_arr_mut_ref[0] == char_arr_mut_ref[1]
}