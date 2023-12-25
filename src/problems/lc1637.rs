//! ## Leetcode 1637. Widest Vertical Area Between Two Points Containing No Points
//! https://leetcode.com/problems/widest-vertical-area-between-two-points-containing-no-points
//! - `Medium`; `Independently Solved`; `2023-12-20`;
//! 
//! I'm practicing using `map` to perform some handling. I'm pretty surperised that the `map` function with `collect` returns a `vec` with the correct capacity. I'm curious how that was done. Although implementations can have a variable storing size, handling filters and complex data structures can still be tricky.

fn get_x_with_map(points: &Vec<Vec<i32>>) -> Vec<i32> {
  points.iter().map(|v| {
    *v.get(0).expect("Should have a X")
  }).collect()
}

fn get_x(points: &Vec<Vec<i32>>) -> Vec<i32> {
  let len = points.len();
  let mut ans: Vec<i32> = Vec::with_capacity(len);
  for i in points.iter() {
    ans.push(*i.get(0).expect("Should have x"));
  }
  ans
}

pub fn max_width_of_vertical_area(points: Vec<Vec<i32>>) -> i32 {
  let mut xs: Vec<i32> = points.iter().map(|v| {
    *v.get(0).expect("Should have a X")
  }).collect();
  xs.sort();
  let mut iter = xs.iter();
  let mut prev = iter.next().expect("Should have at least one element");
  let mut ans: i32 = 0;
  for i in iter {
    ans = ans.max(*i - *prev);
    prev = i;
  }
  ans
}

#[cfg(test)]
mod test {
  use super::*;
  use crate::helpers::test::vec_capa_matches_len;
  #[test]
  fn get_x_with_map_capacity_correct() {
    let points: Vec<Vec<i32>> = vec![vec![1,2], vec![3,4], vec![5,6], vec![7,8],  vec![9,10]];
    let ans = get_x_with_map(&points);
    
    assert!(vec_capa_matches_len(&ans));
  }

  #[test]
  fn get_x_capacity_correct() {
    let points: Vec<Vec<i32>> = vec![vec![1,2], vec![3,4], vec![5,6]];
    let ans = get_x(&points);
    assert!(vec_capa_matches_len(&ans));
  }
}