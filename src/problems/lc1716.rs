//! ## Leetcode 1716. Calculate Money in Leetcode Bank
//! https://leetcode.com/problems/calculate-money-in-leetcode-bank
//! - `Easy`; `Independently Solved`; `2023-12-05`;
//! 
//! This is a problem focusing on arithmetic sums. The differences in deposited money between full weeks are constant, and the differences in deposited money between days within one week are also constant. 

pub fn total_money(n: i32) -> i32 {
    fn sum(base: i32, inc: i32, n: i32) -> i32 {
        ((base + base + inc * (n - 1)) * n) / 2
    }
    let weeks = n / 7;
    sum(sum(1,1,7), 7, weeks) + sum(1 + weeks, 1, n % 7)
}