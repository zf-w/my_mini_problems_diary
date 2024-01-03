//! ## Leetcode 2610. Convert an Array Into a 2D Array With Conditions
//! https://leetcode.com/problems/convert-an-array-into-a-2d-array-with-conditions
//! - `Medium`; `Independently Solved`; `2024-01-01`;
//! 
//! Knowing the `size` of a vector is kind of important.

pub fn find_matrix(nums: Vec<i32>) -> Vec<Vec<i32>> {
  let len = nums.len();
  let mut count: Vec<u16> = vec![0; len];
  
  for num in nums.iter() {
    let curr: &mut u16 = &mut count[(num - 1) as usize];
    *curr += 1;
  }
  let mut curr_size: usize = 0;
  let mut max_count: u16 = 0;
  for n in count.iter() {
    if *n > 0 {
      curr_size += 1;
    }
    max_count = max_count.max(*n);
  }
  let mut res: Vec<Vec<i32>> = Vec::with_capacity(max_count as usize);
  
  for _ in 0..max_count {
    let mut curr: Vec<i32> = Vec::with_capacity(curr_size);
    for (i, n) in count.iter_mut().enumerate() {
      if *n > 0 {
        curr.push((i + 1) as i32);
        *n -= 1;
        if *n == 0 {
          curr_size -= 1;
        }
      }
    }
    res.push(curr);
  }
  res
}

#[cfg(test)]
mod test {
  use crate::helpers::test::vec_capa_matches_len;
  use super::find_matrix;

  #[test]
  fn check_capacity() {
    let nums = vec![1,3,4,1,2,3,1];
    let res = find_matrix(nums);
    vec_capa_matches_len(&res);
    for sub in res.iter() {
      vec_capa_matches_len(sub);
    }
  }
}