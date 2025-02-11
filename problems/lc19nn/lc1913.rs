//! ## Leetcode 1913. Maximum Product Difference Between Two Pairs
//! https://leetcode.com/problems/maximum-product-difference-between-two-pairs
//! - `Easy`; `Independently Solved`; `2023-12-17`;
//! 
//! Playing with class definition with template and traits. I guess using a set would make the "struct" more general.

struct Top2<T> where
  T: Ord + Clone
{
  n0: Option<T>,
  n1: Option<T>
}

impl<T> Top2<T> where
  T: Ord + Clone
{
  pub fn new() -> Self {
    Self {
      n0: None,
      n1: None
    }
  }
  pub fn add(&mut self, n: T) {
    if let Some(n1) = self.n1.clone() {
      if n > n1 {
        self.n1 = Some(n.clone());
      }
    } else {
      self.n1 = Some(n.clone());
    }
    if let Some(n0) = self.n0.clone() {
      if n > n0 {
        self.n1 = Some(n0);
        self.n0 = Some(n);
      }
    } else {
      self.n0 = Some(n);
      self.n1 = None;
    }
  }

  pub fn res(&self) -> (T, T) {
    (self.n0.clone().unwrap(), self.n1.clone().unwrap())
  }
}

pub fn max_product_difference(nums: Vec<i32>) -> i32 {
  let mut max2: Top2<i32> = Top2::new();
  let mut min2: Top2<i32> = Top2::new();
  for num in nums.iter() {
    max2.add(num.clone());
    min2.add(-num.clone());
  }
  let (n0, n1) = max2.res();
  let (n2, n3) = min2.res();

  n0 * n1 + n2 * n3
}