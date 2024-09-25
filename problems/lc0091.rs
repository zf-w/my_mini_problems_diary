//! ## Leetcode 91. Decode Ways
//! https://leetcode.com/problems/decode-ways
//! - `Medium`; `Independently Solved`; `2023-12-24`;
//! 
//! It's been a long time since the last problem with Dynamic Programming. I guess a bottom-up approach would be more time-and-memory-efficient.

use std::collections::HashMap;

pub fn num_decodings(s: String) -> i32 {
  fn parse(s: &str) -> bool {
      // println!("{}", s);
      match s.parse::<i32>() {
          Ok(n) => {
              if n >= 1 && n < 27 && !(s.len() == 2 && n < 10) {
                  true
              } else {
                  false
              }
          },
          Err(_) => false
      }
  }
  fn helper<'a>(s: &'a str, m: &mut HashMap<&'a str, i32>) -> i32 {
    if let Some(ans) = m.get(s) {
      return *ans;
    }
    if s.len() == 0 {
      m.insert(s, 1);
      return 1;
    }
    let mut ans: i32 = 0;
    if parse(&s[0..1]) {
        ans += helper(&s[1..], m);
    }
    if s.len() >= 2 && parse(&s[0..2]) {
        ans += helper(&s[2..], m);
    }
    m.insert(s, ans);
    ans
  }
  let mut m: HashMap<&str, i32> = HashMap::new();
  helper(s.as_str(), &mut m)
}