//! ## Leetcode 1561. Maximum Number of Coins You Can Get
//!  https://leetcode.com/problems/maximum-number-of-coins-you-can-get/
//! 
//! - `Medium` 
//! 
//! There are `3n` piles of coins of varying size, you and four friends will take piles of coins as follows:
//! 
//! - In each step, you will choose **any** `3` piles of coins (not necessarily consecutive).
//! - Of your choice, Alice will pick the pile with the maximum number of coins.
//! - You will pick the next pile with the maximum number of coins.
//! - Your friend Bob will pick the last pile.
//! - Repeat until there are no more piles of coins.
//! 
//! Given an array of integers `piles` where `piles[i]` is the number of coins in the `i^th` pile.
//! 
//! Return the *maximum number of coins that you can have*.
//! 
//! ### Example:
//! 
//! ```
//! use learn_cs::problems::lc1561;
//! let mut piles = vec![2, 4, 1, 2, 7, 8];
//! 
//! assert_eq!(9, lc1561::max_coins(piles)); 
//! ```
//! 
//! ### Thoughts:
//! - `Independently Solved`; `2023-11-23`
//! 
//! To maximize your number of coins, I think the only thing you can do is to minimize the coins obtained by Bob. I'm so sorry for Bob. Hopefully, I will not meet people with such evil mindsets.

pub fn max_coins(mut piles: Vec<i32>) -> i32 {
  piles.sort();
  let len = piles.len();
  let len_me = len / 3;
  let mut ans: i32 = 0;
  for i in 0..len_me {
    ans += piles[len - 2 - i * 2];
  }
  ans
}