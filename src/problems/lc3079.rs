//! ## Leetcode 3079. Find the Sum of Encrypted Integers
//! https://leetcode.com/problems/find-the-sum-of-encrypted-integers
//! - `Easy`; `Independently Solved`; `2024-03-16`;
//!
//! This question is the first question of Leetcode Biweekly Contest 126.
//!
//! A plain simulation would work.

pub fn sum_of_encrypted_int(nums: Vec<i32>) -> i32 {
    fn encrypt(mut num: i32) -> i32 {
        let mut curr_max = 0;
        let mut count = 0;
        while num > 0 {
            curr_max = curr_max.max(num % 10);
            num /= 10;
            count = count * 10 + 1;
        }
        count * curr_max
    }
    let mut sum = 0;
    for num in nums {
        sum += encrypt(num);
    }
    sum
}
