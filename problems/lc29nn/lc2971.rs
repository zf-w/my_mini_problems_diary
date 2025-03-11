//! ## Leetcode 2971. Find Polygon With the Largest Perimeter
//! https://leetcode.com/problems/find-polygon-with-the-largest-perimeter
//! - `Medium`; `Independently Solved`; `2024-02-15`;
//!
//! I think if the longest edge is shorter than the sum of the rest of the edges, those edges can form a polygon.

pub fn largest_perimeter(mut nums: Vec<i32>) -> i64 {
    nums.sort();
    let mut ans = 0;
    let mut prev_sum: i64 = 0;
    for num in nums.iter() {
        let curr = *num as i64;
        if prev_sum > curr {
            ans = prev_sum + curr;
        }
        prev_sum = prev_sum + curr;
    }
    ans
}
