//! ## Leetcode 2706. Buy Two Chocolates
//! https://leetcode.com/problems/buy-two-chocolates
//! - `Easy`; `Independently Solved`; `2023-12-19`;
//! 
//! I guess finding the first `k` based on their priorities from an iterator is a very common need. The basic idea would be using a priority queue to store the `top k` elements and popping the element with the lowest priority when having already `k` elements.

pub fn buy_choco(prices: Vec<i32>, money: i32) -> i32 {
  let mut min0: Option<i32> = None;
  let mut min1: Option<i32> = None;

  for price in prices.iter() {
    if let Some(v) = min1 {
      if *price < v {
        min1 = Some(*price);
      }
    } else {
      min1 = Some(*price);
    }
    if let Some(v) = min0 {
      if *price < v {
        min1 = min0;
        min0 = Some(*price);
      }
    } else if min1.is_some() {
      min0 = min1;
      min1 = None;
    }
  }
  if let (Some(v0), Some(v1))= (min0, min1) {
    if v1 + v0 < money {
      money - v0 - v1
    } else {
      money
    }
  } else {
    money
  }
}