//! # Leetcode 3304. Find the K-th Character in String Game I
//! https://leetcode.com/problems/find-the-k-th-character-in-string-game-i/
//! - `Easy`; `y2025m07d03`; `Independently Solved`; `N/A`; `N/A`; `1 attempt`;
//! Topics: observe_patterns.

pub fn kth_character(k: i32) -> char {
    (b'a' + (k - 1).count_ones() as u8) as char
}
