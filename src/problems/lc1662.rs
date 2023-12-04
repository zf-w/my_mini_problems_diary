//! ## Leetcode 1662. Check If Two String Arrays are Equivalent
//! https://leetcode.com/problems/check-if-two-string-arrays-are-equivalent
//! - `Easy`; `Independently Solved`; `2023-11-30`;
//! 
//! That's actually an very interesting question due to the complex nature of strings. I guess strings are not exactly character arrays. Instead, they are more similar to **Linked Lists** since the length of a character is not fixed. 
//! 
//! So I guess I cannot frequently use the subscription operator and have to figure something out using iterators. 


pub fn array_strings_are_equal(word1: Vec<String>, word2: Vec<String>) -> bool {
      let mut i = word1.iter();
      let mut j = word2.iter();
      let mut i0 = i.next();
      let mut j0 = j.next();
      if i0.is_none() != j0.is_none() {
          return false;
      }
      let mut i0_iter = i0.unwrap().chars();
      let mut j0_iter = j0.unwrap().chars();

      loop {
          let next = (i0_iter.next(), j0_iter.next());
          match next {
              (Some(c0), Some(c1)) => {
                  if c0 != c1 {
                      return false;
                  }
              },
              (Some(c0), None) => {
                  j0 = j.next();
                  if j0.is_none() {
                      return false;
                  }
                  j0_iter = j0.unwrap().chars();
                    if let Some(c1) = j0_iter.next() {
                      if c0 != c1 {
                          return false;
                      }
                  } else {
                      return false;
                  }
              },
              (None, Some(c1)) => {
                  i0 = i.next();
                  if i0.is_none() {
                      return false;
                  }
                  i0_iter = i0.unwrap().chars();
                    if let Some(c0) = i0_iter.next() {
                      if c0 != c1 {
                          return false;
                      }
                  } else {
                      return false;
                  }
              },
              (None, None) => {
                  i0 = i.next();
                  j0 = j.next();
                  if i0.is_none() != j0.is_none() {
                      return false;
                  } else if i0.is_none() == true {
                      return true;
                  }
                  i0_iter = i0.unwrap().chars();
                  j0_iter = j0.unwrap().chars();
                  println!("ij jumping");
              }
          }
      }
  }