//! ## Leetcode 445. Assign Cookies
//! https://leetcode.com/problems/assign-cookies
//! -`Easy`; `Independently Solved`; `2023-12-31`;
//! 
//! I guess a binary heap would also work.

pub fn find_content_children(mut g: Vec<i32>, mut s: Vec<i32>) -> i32 {
  g.sort();
  s.sort();
  let mut count: i32 = 0;
  let mut cookie_iter = s.iter();
  for child in g.iter() {
    while let Some(cookie) = cookie_iter.next() {
      if *cookie >= *child {
        count += 1;
        break;
      }
    }
  }
  count
}