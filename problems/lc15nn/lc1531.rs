//! ## Leetcode 1531. String Compression ||
//! https://leetcode.com/problems/string-compression-ii
//! - `Hard`; `Learned from previous`; `2023-12-27`;
//! 
//! I still didn't fully get the idea of the solution, but this question is indeed a very comprehensive chance for me to practice with various Rust ideas and features.
//! 
//! The String data structure is indeed very interesting. In theory, a UTF-8 string should be read in a linked-list manner, which is super costly in time (O(n)). For this question, the linked-list reading style took 700ms while the array reading style only took 18ms. 

pub fn get_length_of_optimal_compression(s: String, k: i32) -> i32 {
  fn digit(mut n: usize) -> i32 {
    let mut count = 0;
    while n > 0 {
      count += 1;
      n /= 10;
    }
    count
  }
  let s_raw = s.as_bytes();
  let off = k as usize + 1;
  let index = |i: usize, j: i32| -> usize {
    i * off + j as usize
  };
  let len = s.len();
  let mut dp: Vec<i32> = vec![0; (len + 1) * off];
  let mut i_iter = s.char_indices().rev();
  while let Some((i, i_char)) = i_iter.next() {
    for j in 0..=k {
      let dp_idx = index(i, j);
      dp[dp_idx] = if j > 0 {
        dp[index(i + 1, j - 1)]
      } else {
        105
      };
      let mut possible_del = j;
      let mut count = 0;
      let mut end_idx = i;
      
      while end_idx < len&& possible_del >= 0 {
        
        let end = s_raw[end_idx] as char;
        if end == i_char {
          count += 1;

          dp[dp_idx] = dp[dp_idx].min(dp[index(end_idx, possible_del)] + digit(count) + 1);
        } else {
          possible_del -= 1;
        }
        end_idx += 1;
      }
    }
  }
  dp[k as usize]
}