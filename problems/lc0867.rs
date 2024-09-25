//! ## Leetcode 867. Transpose Matrix
//! https://leetcode.com/problems/transpose-matrix
//! - `Easy`; `Independently Solved`; `2023-12-09`;
//! 
//! This reminds me of my thought about using a 1D array to represent a rectangular 2D array. I guess, if possible, any array structure with a fixed size should be somehow represented in the form of a 1D array. Even if the array is not in a regular shape, the sizes of arrays in the next "dimension" are all not equal, I guess it would be possible to use an additional prefix sum array to record the size of all previous layers for quickly accessing elements.

pub fn transpose(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
  let m = matrix.len();
  let n = matrix[0].len();
  let mut ans: Vec<Vec<i32>> = Vec::with_capacity(n);
  for _ in 0..n {
    ans.push(Vec::with_capacity(m));
  }
  for (_, row) in matrix.iter().enumerate() {
    for (j, cell) in row.iter().enumerate() {
      ans[j].push(*cell);
    }
  }
  ans
}