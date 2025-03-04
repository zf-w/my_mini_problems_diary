//! ## Leetcode 1624. Largest Substring Between Two Equal Characters
//! https://leetcode.com/problems/largest-substring-between-two-equal-characters
//! - `Easy`; `Independently Solved`; `2023-12-30`;
//! 
//! Checking the previous position of the same character.

pub fn max_length_between_equal_characters(s: String) -> i32 {
  let len = s.len();
  let mut first: [usize; 26] = [len; 26];
  let mut ans = 0;
  let base = 'a' as usize;
  for (i, c) in s.chars().enumerate() {
    let c_i = c as usize - base;
    if first[c_i] < i {
      ans = ans.max((i - first[c_i]) as i32);
    } else {
      first[c_i] = i;
    }
  }
  ans
}