//! ## Leetcode 1291. Sequential Digits
//! https://leetcode.com/problems/sequential-digits
//! - `Medium`; `Independently Solved`; `2023-02-02`;
//!
//! Just iterating through possible results. I guess there might be a way to push the satisfied integers in the sorted order, like by iterating with the length of the integer.

pub fn sequential_digits(low: i32, high: i32) -> Vec<i32> {
    let mut ans: Vec<i32> = Vec::new();
    for start in 1..9 {
        let mut val = start;
        let mut curr = start;
        while val <= high && curr < 10 {
            if val >= low {
                ans.push(val);
            }
            curr += 1;
            val = val * 10 + curr;
        }
    }
    ans.sort();
    ans
}
