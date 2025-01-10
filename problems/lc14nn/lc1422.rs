//! ## Leetcode 1422. Maximum Score After Splitting a String
//! https://leetcode.com/problems/maximum-score-after-splitting-a-string
//! - `Easy`; `Independently Solved`; `2023-12-21`;
//! 
//! I'm curious about the time complexity of the subscription operation. In theory, for a "linked list" like "String" data structure, it would be costly to check where the last element is.

pub fn max_score(s: String) -> i32 {
  let mut curr: i32 = 0;
  for c in s.chars() {
    match c {
      '1' => {
        curr += 1;
      },
      _ => ()
    }
  }
  let mut ans = -1;
  for c in s[0..s.len() - 1].chars() {
    match c {
      '1' => {
        curr -= 1;
      },
      '0' => {
        curr += 1;
      }
      _ => ()
    }
    ans = ans.max(curr);
  }
  ans
}