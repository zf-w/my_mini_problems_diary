//! ## Leetcode 1582. Special Positions in a Binary Matrix
//! https://leetcode.com/problems/special-positions-in-a-binary-matrix
//! - `Easy`; `Independently Solved`; `2023-12-12`;
//! 
//! I guess solving this needs `O(m * n)`.

pub fn num_special(mat: Vec<Vec<i32>>) -> i32 {
  let m = mat.len();
  let n = mat[0].len();
  let mut h: Vec<bool> = vec![false; m];
  let mut v: Vec<bool> = vec![false; n];
  for i in 0..m {
      let mut count = 0;
      for j in 0..n {
          if mat[i][j] == 1 {
              count += 1;
              if count > 1 {
                  break;
              }
          }
      }
      if count == 1 {
          h[i] = true;
      }
  }

  for j in 0..n {
      let mut count = 0;
      for i in 0..m {
          if mat[i][j] == 1 {
              count += 1;
              if count > 1 {
              break;
              }
          }
      }
      if count == 1 {
          v[j] = true;
      }
      // print!("{} ", v[j]);
  }
  let mut ans = 0;
  for (i, row) in mat.iter().enumerate() {
      for (j, cell) in row.iter().enumerate() {
          if *cell == 1 && h[i] == true && v[j] == true {
              ans += 1;
          }
      }
  }
  ans
}