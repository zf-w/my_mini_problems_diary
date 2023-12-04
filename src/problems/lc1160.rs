//! ## Leetcode 1160. Find Words That Can Be Formed By Characters
//! https://leetcode.com/problems/find-words-that-can-be-formed-by-characters
//! - `Easy`; `Independently Solved`; `2023-12-01`;
//! 
//! I think sometimes I forget to use an array and use a vector, even for fixed-size linear structures. I guess I should not use dynamic heap memories when I know what the size of an array would be.
//! 

pub fn count_characters(words: Vec<String>, chars: String) -> i32 {
  let mut counts: [usize; 26] = [0; 26];
  let mut cmp: [usize; 26] = [0; 26];
  fn clear(cmp: &mut [usize; 26]) {
    let len: usize = 26;
    for i in 0..len {
      cmp[i] = 0;
    }
  }
  let c_base = 'a' as usize;
  for c in chars.chars() {
    counts[c as usize - c_base] += 1;
  }
  let mut ans: i32 = 0;
  for word in words.iter() {
    for c in word.chars() {
      cmp[c as usize - c_base] += 1;
    }
    let mut good = true;
    let len: usize = 26;
    for i in 0..len {
      if cmp[i] > counts[i] {
        good = false;
        break;
      }
    }
    if good {
      ans += word.len() as i32;
    }
  }
  ans
}