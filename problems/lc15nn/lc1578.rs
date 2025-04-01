//! ## Leetcode 1578. Minimum Time to Make Rope Colorful
//! https://leetcode.com/problems/minimum-time-to-make-rope-colorful
//! - `Medium`; `Previous`; `2023-12-26`;
//! 
//! In the beginning, I thought this problem was about Dynamic Programming. I found this problem was actually about picking the most costly ballon from consecutive groups of ballons of the same color.

pub fn min_cost(colors: String, needed_time: Vec<i32>) -> i32 { 
  let mut c_iter = colors.chars();
  let mut t_iter = needed_time.iter();
  let mut prev = c_iter.next().expect("at least one element");
  let mut max = *t_iter.next().expect("at least one element");

  let mut sum = max;
  let mut count: usize = 1;
  let mut ans: i32 = 0;
  for (c, t) in c_iter.zip(t_iter) {
    if c == prev {
      max = max.max(*t);
      sum += t;
      count += 1;
    } else {
      if count > 1 {
        ans += sum - max;
      }
      prev = c;
      sum = *t;
      max = *t;
      count = 1;
    }
  }
  if count > 1 {
    ans += sum - max;
  }
  ans
}