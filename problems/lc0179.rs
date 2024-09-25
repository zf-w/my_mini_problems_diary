//! ## Leetcode 179. Largest Number
//! https://leetcode.com/problems/largest-number/
//! - `Medium`; `Learned from Solution`; `2024-09-17`;
//!
//! Learned from https://leetcode.com/problems/largest-number/solutions/5802128/sort-strings-w-r-t-the-proper-ordering

use std::cmp::Ordering;
impl Solution {
    pub fn largest_number(mut nums: Vec<i32>) -> String {
        if nums
            .iter()
            .fold(true, |acc, elem_ref| -> bool { acc && *elem_ref == 0 })
        {
            return "0".to_string();
        }
        fn cmp_num(a: &i32, b: &i32) -> Ordering {
            if *a == 0 || *b == 0 {
                return a.cmp(b);
            }
            let a_first = *a as u64 * 10u64.pow(b.ilog10() + 1);
            let b_first = *b as u64 * 10u64.pow(a.ilog10() + 1);
            (a_first + *b as u64).cmp(&(b_first + *a as u64))
        }
        fn digit_to_char(digit: i32) -> char {
            const ZERO_U8_BASE: u8 = '0' as u8;
            (ZERO_U8_BASE + digit as u8) as char
        }
        nums.sort_unstable_by(cmp_num);
        let mut ans_string = String::new();
        for mut num in nums {
            if num == 0 {
                ans_string.push('0');
            }
            while num > 0 {
                ans_string.push(digit_to_char(num % 10));
                num /= 10;
            }
        }
        ans_string.chars().rev().collect()
    }
}
