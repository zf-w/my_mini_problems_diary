//! ## Leetcode 661. Image Smoother
//! https://leetcode.com/problems/image-smoother
//! - `Easy`; `Independently Solved`; `2023-12-18`;
//! 
//! The problem is basically about simulating a convolutional kernel. Looping through the matrix with padding can be annoying. I found using `max` and `min` to bound the loop instead of using `if` and `continue` can be pretty convenient.

pub fn image_smoother(img: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
  let row_len = img.len();
  let col_len = img.get(0).expect("Expect to have a row").len();
  let mut ans: Vec<Vec<i32>> = Vec::with_capacity(row_len);
  fn get_value(i: usize, j: usize, img: &Vec<Vec<i32>>) -> i32 {
    let row_len = img.len();
    let col_len = img.get(0).expect("Expect to have a row").len();
    let mut count: usize = 0;
    let mut sum: i32 = 0;
    for i1 in (0.max(i as i32 - 1) as usize)..(row_len).min(i + 2) {
      for j1 in (0.max(j as i32 - 1) as usize)..(col_len).min(j + 2) {
        sum += img.get(i1).expect("Should have a row")
          .get(j1).expect("Should have a cell");
        count += 1;
      }
    }
    sum / count as i32
  }
  for i in 0..row_len {
    let mut curr_row = Vec::with_capacity(col_len);
    for j in 0..col_len {
      curr_row.push(get_value(i, j, &img));
    }
    ans.push(curr_row);
  }
  ans
}