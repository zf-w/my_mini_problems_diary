//! ## Leetcode 1436. Destination City
//! https://leetcode.com/problems/destination-city
//! - `Easy`; `Independently Solved`; `2023-12-14`;
//! 
//! I think using the "get + expect" is more natural compared to a plain subscription. At least I can provide more explanation for my implementation with the message "expect."

use std::collections::HashMap;

pub fn dest_city(paths: Vec<Vec<String>>) -> String {
  let len: usize = paths.len();
  let mut count: HashMap<String, i32> = HashMap::with_capacity(len * 2);
  for path in paths.iter() {
    let a = path.get(0).expect("Should have a element");
    if let Some(v) = count.get_mut(a) {
      *v -= 1;
    } else {
      count.insert(a.clone(), -1);
    }
    let b = path.get(1).expect("Should have a second value");
    if let Some(v) = count.get_mut(b) {
      *v += 1;
    } else {
      count.insert(b.clone(), 1);
    }
  }
   let mut ans: String = String::from("");
  for (k, v) in count.iter() {
    if *v == 1 {
      ans = k.clone();
      break;
    }
  }
  ans
}