//! ## Leetcode 1424. Diagonal Traverse II
//! - `Medium`
//! 
//! Given a 2D integer array `nums`, return *all elements of `nums` in diagonal order as shown in the below images*.
//! 
//! ### Example:
//! 
//! |row/col| 0 | 1 | 2 | 4 | 5 |
//! | --| -- | -- | -- | -- | -- |
//! | 0 | 1 | 2 | 3 | 4 | 5 |
//! | 1 | 6 | 7 | | | |
//! | 2 | 8 | | | | |
//! | 3 | 9 | 10 | 11 | | |
//! | 4 | 12 | 13 | 14 | 15 | 16 |
//! 
//! ```
//! use learn_cs::problems::lc1424;
//! let nums = vec![vec![1,2,3,4,5],vec![6,7],vec![8],vec![9,10,11],vec![12,13,14,15,16]];
//! let expected = vec![1,6,2,8,7,3,9,4,12,10,5,13,11,14,15,16];
//! assert_eq!(expected, lc1424::find_diagonal_order(nums));
//! ```
//! 
//! ### Thoughts:
//! - `Independently Solved`; `2023-11-21`;
//! 
//! My first intuition to solve this problem was to make a somewhat square iteration. But I guess that approach would be too time-consuming for a L-shaped array in `O(n^2)`. Then I figured out that sorting with its "diagonal index" would be better, only `O(nlog(n))`.


pub fn find_diagonal_order(nums: Vec<Vec<i32>>) -> Vec<i32> {
  let mut total_len: usize = 0;
  for row in nums.iter() {
    total_len += row.len();
  }
  let mut helper: Vec<(usize, usize, i32)> = Vec::with_capacity(total_len);
  for (i, row) in nums.iter().enumerate() {
    for (j,cell) in row.iter().enumerate() {
      helper.push((i + j, j, *cell));
    }
  }
  helper.sort();
  let mut ans: Vec<i32> = Vec::with_capacity(total_len);
  for (_, _, cell) in helper.iter() {
    ans.push(*cell);
  }
  ans
}