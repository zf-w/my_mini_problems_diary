//! # Leetcode 1780. Check if Number is a Sum of Powers of Three
//! https://leetcode.com/problems/check-if-number-is-a-sum-of-powers-of-three/
//! - `Medium`; `y2025m03d04`; `Independently Solved`; `0ms`; `2.2mb`; `1 attempt`;
//! Topics: uncategorized.

pub fn check_powers_of_three(mut n: i32) -> bool {
    while (n > 0) {
        if (n % 3 == 2) {
            return false;
        }
        n /= 3;
    }
    true
}
