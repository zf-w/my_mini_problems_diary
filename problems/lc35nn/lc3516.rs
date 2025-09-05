//! # Leetcode 3516. Find Closest Person
//! https://leetcode.com/problems/find-closest-person/
//! - `Easy`; `y2025m09d04`; `Independently Solved`; `0ms`; `2.1mb`; `1 attempt`;
//! Topics: uncategorized.

pub fn find_closest(x: i32, y: i32, z: i32) -> i32 {
    match (x - z).abs().cmp(&(y - z).abs()) {
        std::cmp::Ordering::Less => 1,
        std::cmp::Ordering::Equal => 0,
        std::cmp::Ordering::Greater => 2,
    }
}
