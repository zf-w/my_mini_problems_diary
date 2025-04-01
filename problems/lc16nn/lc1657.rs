//! ## Leetcode 1657. Determine if Two Strings Are Close
//! https://leetcode.com/problems/determine-if-two-strings-are-close
//! - `Medium`; `Independently Solved`; `2024-01-13`;
//!
//! Check if the character frequency distribution is the same. Be careful; the existing character sets also need to match since you can only swap between existing characters.

pub fn close_strings(word1: String, word2: String) -> bool {
    let mut count1: [usize; 26] = [0; 26];
    let mut count2: [usize; 26] = [0; 26];
    fn count_word(word: &str, count: &mut [usize; 26]) {
        let base = 'a' as usize;
        for c in word.chars() {
            count[c as usize - base] += 1;
        }
    }
    count_word(&word1, &mut count1);
    count_word(&word2, &mut count2);
    for i in 0..26usize {
        if count1[i] != count2[i] {
            return false;
        }
    }
    count1.sort();
    count2.sort();
    count1 == count2
}
