//! # Leetcode 2843. Count Symmetric Integers
//! https://leetcode.com/problems/count-symmetric-integers/
//! - `Easy`; `y2025m04d11`; `Independently Solved`; `7ms`; `2.2mb`; `1 attempt`;
//! Topics: uncategorized.
//!
//! Busy these days. I copied from the solution first and then solved it on my own.

pub fn count_symmetric_integers(low: i32, high: i32) -> i32 {
    fn check_num(num: i32) -> bool {
        let mut num_copy = num;
        let mut digit_num: usize = 0;
        while num_copy > 0 {
            num_copy /= 10;
            digit_num += 1;
        }

        if digit_num & 1 == 1 {
            return false;
        }

        let mut high = 10_i32.pow(digit_num as u32);
        let mut low = 1;

        let mut diff = 0;

        while low < high {
            diff += (num / low) % 10 - (num / high) % 10;

            low *= 10;
            high /= 10;
        }
        diff == 0
    }
    let mut ans_count = 0;
    for num in low..=high {
        if check_num(num) {
            ans_count += 1;
        }
    }
    ans_count
}
