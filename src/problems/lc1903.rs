//! ## Leetcode 1903. Largest Odd Number in String
//! https://leetcode.com/problems/largest-odd-number-in-string
//! - `Easy`; `Independently Solved`; `2023-12-06`;
//! 
//! I wonder why the recent Leetcode daily problems are all easy. They are probably considering the final week for students. So considerate... I also wonder what is the implementation for `rev()` is. I think it's possible to read a string-byte array from behind.

pub fn largest_odd_number(num: String) -> String {
  let base: usize = '0' as usize;
  let mut end: usize = 0;
  for (i, c) in num.char_indices().rev() {
    if (c as usize - base) & 1 == 1 {
      end = i + 1;
      break;
    }
  }
  num[0..end].to_string()
}