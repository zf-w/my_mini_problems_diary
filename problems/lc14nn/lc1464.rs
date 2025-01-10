//! ## Leetcode 1464. Maximum Product of Two Elements in an Array
//! https://leetcode.com/problems/maximum-product-of-two-elements-in-an-array
//! - `Easy`; `Independently Solved`; `2023-12-11`;
//! 
//! In general, I think it's possible to make selecting some number of top elements into a method of iterators.

pub fn max_product(nums: Vec<i32>) -> i32 {
  let mut n0 = 1;
  let mut n1 = 1;
  for num in nums.iter() {
    if *num > n1 {
      if *num > n0 {
        n1 = n0;
        n0 = *num;
      } else {
        n1 = *num;
      }
    }
  }        
  (n0 - 1) * (n1 - 1)
}