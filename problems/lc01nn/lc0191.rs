//! Leetcode 191. Number of 1 Bits
//! https://leetcode.com/problems/number-of-1-bits
//! - `Easy`; `Independently Solved`; `2023-11-28`;
//! 
//! The 1 Bit counting problem. I used to iterate through all the bits by bit-operations or divide. I have learned the more efficient method via a Connect 4 Solving tutorial. This method guarantees to find the rightmost bit of 1 quickly.

pub fn hamming_weight (mut n: u32) -> i32 {
    let mut count: i32 = 0;
    while n > 0 {
        n &= n - 1;
        count += 1;
    }
    count
}