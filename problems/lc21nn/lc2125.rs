//! ## Leetcode 2125. Number of Laser Beams in a Bank
//! https://leetcode.com/problems/number-of-laser-beams-in-a-bank
//! - `Medium`; `Independently Solved`; `2024-01-02`;
//! 
//! This problem reminds me of neural networks and matrices.

pub fn number_of_beams(bank: Vec<String>) -> i32 {
  let mut floor_iter = bank.iter();
  let is_one = |x: &char| -> bool {
    x == &'1'
  };
  let mut prev = if let Some(curr) = floor_iter.next() {
    curr.chars().filter(is_one).count() as i32
  } else {
    0
  };
  let mut ans = 0;
  for curr_str in floor_iter {
    let curr = curr_str.chars().filter(is_one).count() as i32;
    if curr > 0 {
      ans += prev * curr;
      prev = curr;
    }
  }

  ans
}