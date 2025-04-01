//! ## Leetcode 1561. Maximum Number of Coins You Can Get
//!  https://leetcode.com/problems/maximum-number-of-coins-you-can-get
//! - `Medium`; `Independently Solved`; `2023-11-23`;
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