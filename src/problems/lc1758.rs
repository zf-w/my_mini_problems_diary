//! ## Leetcode 1758. Minimum Changes To Make Alternating Binary String
//! https://leetcode.com/problems/minimum-changes-to-make-alternating-binary-string
//! - `Easy`; `Independently Solved`; `2023-12-23`;
//! 
//! There are only two possible binary strings: one starts with zero, and one starts with one.

pub fn min_operations(s: String) -> i32 {
  let mut start_0 = 0;
  let mut start_1 = 0;
  for (i,c) in s.char_indices() {
    if i % 2 == 0 {
      if c == '0' {
        start_1 += 1;
      } else {
        start_0 += 1;
      }
    } else {
      if c == '1' {
        start_1 += 1;
      } else {
        start_0 += 1;
      }
    }
  }
  start_0.min(start_1)       
}