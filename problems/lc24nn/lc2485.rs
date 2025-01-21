//! ## Leetcode 2485. Find the Pivot Integer
//! https://leetcode.com/problems/find-the-pivot-integer
//! - `Easy`; `Independently Solved`; `2024-03-12`;
//!
//! Solving with binary search might be another way.

pub fn pivot_integer(n: i32) -> i32 {
    let ans = f32::sqrt((n * (n + 1) / 2) as f32);
    if ans - ans.floor() == 0.0 {
        ans as i32
    } else {
        -1
    }
}
