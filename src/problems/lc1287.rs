//! Leetcode 1287. Element Appearing More Than 25% In Sorted Array
//! https://leetcode.com/problems/element-appearing-more-than-25-in-sorted-array
//! - `Easy`; `Independently Solved`; `2023-12-10`;
//! 
//! I guess there are still some possible optimizations, like immediately returning after finding the count of some element is larger than 25%. I guess I have caught something or ate too many peanuts. My throat is feeling funny.

pub fn find_special_integer(arr: Vec<i32>) -> i32 {
  let mut iter = arr.iter().enumerate();
  let mut prev = *iter.next().unwrap().1;
  let mut curr_len: usize = 0;
  let mut ans_len: usize = 1;
  let mut ans = prev;
  for (_, v) in iter {
    if *v == prev {
      curr_len += 1;
    } else {
      if curr_len > ans_len {
        ans_len = ans_len.max(curr_len);
        ans = prev;
      }
      curr_len = 1;
      prev = *v;
    }
  }
  if curr_len > ans_len {
    ans = prev;
  }
  ans
}