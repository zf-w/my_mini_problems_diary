//! ## Leetcode 1266. Minimum Time Visiting All Points
//! https://leetcode.com/problems/minimum-time-visiting-all-points
//! - `Easy`; `Independently Solved`; `2023-12-03`;
//! 
//!  I feel like the key point of this question would be to make as many diagonal moves as possible. I'm also wondering what the best practice is for iterating through a list. Love the idea of abstracting with no performance cost. I guess using the iterator approach would be more robust if switching to a structure with a linked-list-style internal design, say string.

pub fn min_time_to_visit_all_points(points: Vec<Vec<i32>>) -> i32 {
  fn dis(a: &Vec<i32>, b: &Vec<i32>) -> i32 {
    (a[0] - b[0]).abs().max((a[1] - b[1]).abs())
  }
  let mut ans: i32 = 0;
  let mut i = points.iter();
  let mut prev: &Vec<i32> = i.next().unwrap();

  for curr in i {
    ans += dis(prev, curr);
    prev = curr;
  }
  ans
}