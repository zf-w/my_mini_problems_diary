//! ## Leetcode 1155. Number of Dice Rolls With Target Sum
//! https://leetcode.com/problems/number-of-dice-rolls-with-target-sum
//! - `Medium`; `Learned from previous`; `2023-12-25`;
//! 
//! It has been a long time since I saw the last Dynamic Programming with a bottom-up approach. Picking up the idea of using a 1-D array to represent a 2-D Dynamic Programming table took me some time.

pub fn num_rolls_to_target(n: i32, k: i32, target: i32) -> i32 {
  let m: i64 = 1000000007;
  let mut dp: Vec<i64> = vec![0; ((n + 1) * (target + 1)) as usize];
  *dp.get_mut(0).expect("should have an element") = 0;
  for num in 1..=n {
    let mut t = 1;
    while t <= target && t <= num * k {
      for face in 1..=k {
        if t - face >= 0 {
          let curr_idx = (num * t) as usize;
          let prev_idx = ((num - 1) * (t - face)) as usize;
          let val = dp.get(curr_idx).unwrap().clone();
          let prev_val = dp.get(prev_idx).unwrap().clone();
          *dp.get_mut(curr_idx).unwrap() = (val + prev_val) % m;
        }
      }
      t += 1;
    }
  }
  *dp.get((n * k) as usize).expect("should have the final") as i32
}