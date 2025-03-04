//! ## Leetcode 1611. Minimum One Bit Operations to Make Integers Zero
//! https://leetcode.com/problems/minimum-one-bit-operations-to-make-integers-zero
//! - `Hard`; `Learned`; `2023-11-29`;
//! 
//! I didn't fully understand the mechanism of the recursion formula. I improved the leftmost bit-checking part a bit. I guess, to solve problems like this, trying to find the patterns of numbers is indeed important.

pub fn minimum_one_bit_operations(n: i32) -> i32 {
    if n <= 1 {
        return n;
    }
    let mut cp = n;
    while cp & (cp - 1) > 0 {
        cp &= cp - 1;
    }
    return (2 * cp - 1) - self::minimum_one_bit_operations(n - cp);
}