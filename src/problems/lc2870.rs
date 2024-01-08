//! ## Leetcode 2870. Minimum Number of Operations to Make Array Empty
//! https://leetcode.com/problems/minimum-number-of-operations-to-make-array-empty
//! - `Medium`; `Independently Solved`; `2024-01-03`;
//! 
//! The calculation part reminds me of those stone games. Took me some time to figure it out. Super tired today for making the opening book for the Connect4 Solver.

use std::collections::HashMap;

pub fn min_operations(nums: Vec<i32>) -> i32 {
  let mut count: HashMap<i32, i32> = HashMap::new();
  for num in nums.iter() {
    if let Some(v) = count.get_mut(num) {
*v += 1;
    } else {
count.insert(*num, 1);
    }
  }
  let mut ans: i32 = 0;
  fn calcu(n: i32) -> Option<i32> {
    if n == 1 {
      return None;
    }
    let mut num_of_2 = n / 2;
    if n % 2 == 1 {
      num_of_2 -= 1;
      Some(1 + 2 * (num_of_2 / 3) + num_of_2 % 3)
    } else {
      Some(2 * (num_of_2 / 3) + num_of_2 % 3)
    }
  }
  for (_, v) in count.iter() {
    if let Some(n) = calcu(*v) {
    ans += n;
    } else {
      return -1;
    }
  }
  ans
}