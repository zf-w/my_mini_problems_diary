//! ## Leetcode 2264. Largest 3-Same-Digit Number in String
//! https://leetcode.com/problems/largest-3-same-digit-number-in-string
//! - `Easy`; `Independently Solved`; `2023-12-03`
//! 
//! I guess this problem would be another practice of playing around with iterators. Using the iterator's next function to forward it is useful when iterating with the previous element.


pub fn largest_good_integer(num: String) -> String {
  let mut ans: String = String::with_capacity(6);
  if num.len() < 3 {
    return ans;
  }
  let mut iter = num.chars();
  let mut c0 = iter.next().unwrap();
  let mut c1 = iter.next().unwrap();
  let mut ans_num: char = '/';
  for c2 in iter {
    if c0 == c1 && c1 == c2 && c0 > ans_num {
      ans_num = c0;
    }
    c0 = c1;
    c1 = c2;
  }
  if ans_num > '/' {
    ans.push(ans_num);
    ans.push(ans_num);
    ans.push(ans_num);
  }
  ans
}