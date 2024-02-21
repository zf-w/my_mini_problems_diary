//! ## Leetcode 201. Bitwise AND of Numbers Range
//! https://leetcode.com/problems/bitwise-and-of-numbers-range
//! - `Medium`; `Independently Solved`; `2024-02-20`;
//!
//! A quite interesting problem. The key is to loop through each bit position and calculate if a number with a zero bit exists in that position within the range.

pub fn range_bitwise_and(left: i32, right: i32) -> i32 {
    let mut left1: i32 = 0;
    let diff = right - left;
    let mut next: i32 = 1;
    let mut count: i32 = 0;
    let mut i: usize = 0;
    let mut curr: i32 = 1;
    while i < 32 && left >= curr && right >= curr {
        next <<= 1;
        if left & curr > 0 && right & curr > 0 {
            left1 |= left & curr;

            if diff < next - left1 {
                count |= curr;
            }
        }
        curr <<= 1;
        i += 1;
    }
    count
}
