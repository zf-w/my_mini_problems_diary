//! ## Leetcode 1704. Determine if String Halves Are Alike
//! https://leetcode.com/problems/determine-if-string-halves-are-alike
//! - `Easy`; `Independently Solved`; `2024-01-11`;
//!
//! I wonder if the time complexity of using an array of characters is the same as using a bunch of "if else" or "match".

pub fn halves_are_alike(s: String) -> bool {
    fn index(c: char) -> bool {
        let m: [char; 10] = ['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];
        for curr in m.iter() {
            if *curr == c {
                return true;
            }
        }
        false
    }
    let mut count: usize = 0;
    let half = s.len() / 2;
    for (i, c) in s.char_indices() {
        if !index(c) {
            continue;
        }
        if i < half {
            count += 1;
        } else {
            if count > 0 {
                count -= 1;
            } else {
                return false;
            }
        }
    }

    count == 0
}
