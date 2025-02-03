//! ## Leetcode 1424. Diagonal Traverse II
//! https://leetcode.com/problems/diagonal-traverse-ii
//! - `Medium`; `Independently Solved`; `2023-11-21`;
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