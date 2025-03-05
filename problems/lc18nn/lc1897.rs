//! ## Leetcode 1897. Redistribute Characters to Make All Strings Equal
//! https://leetcode.com/problems/redistribute-characters-to-make-all-strings-equal
//! - `Easy`; `Independently Solved`; `2023-12-29`;
//! 
//! Counting if each character's total numbers can be divided is the number of strings.

pub fn make_equal(words: Vec<String>) -> bool {
  let mut count: [usize; 26] = [0; 26];
  let base: usize = 'a' as usize;
  for word in words.iter() {
    for c in word.chars() {
      count[c as usize - base] += 1;
    }
  }
  let len = words.len();
  for i in count.iter() {
    if i % len > 0 {
      return false;
    }
  }
  true
}