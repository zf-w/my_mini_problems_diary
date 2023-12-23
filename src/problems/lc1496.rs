//! ## Leetcode 1496. Path Crossing
//! https://leetcode.com/problems/path-crossing
//! - `Easy`; `Independently Solved`; `2023-12-22`;
//! 
//! Practicing using HashSet and `match`.

use std::collections::HashSet;

pub fn is_path_crossing(path: String) -> bool {
  let mut s: HashSet<(i32, i32)> = HashSet::with_capacity(20);
  let mut curr = (0, 0);
  s.insert(curr.clone());
  for c in path.chars() {
    curr = match c {
      'N' => (curr.0, curr.1 + 1),
      'S' => (curr.0, curr.1 - 1),
      'W' => (curr.0 - 1, curr.1),
      'E' => (curr.0 + 1, curr.1),
      _ => (0, 0)
    };
    if s.contains(&curr) {
      return true;
    } else {
      s.insert(curr);
    }
  }
  false
}