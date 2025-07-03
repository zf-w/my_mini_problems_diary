//! # Leetcode 2311. Longest Binary Subsequence Less Than or Equal to K
//! https://leetcode.com/problems/longest-binary-subsequence-less-than-or-equal-to-k/
//! - `Medium`; `y2025m06d26`; `Hinted`; `0ms`; `2.3mb`; `3 attempts`;
//! Topics: subsequence.

pub fn longest_subsequence(s: String, k: i32) -> i32 {
    let mut digit_num = 1;
    let mut cum = 0;
    let mut ans_len = 0;
    let mut digit_flag = true;
    for is_one_flag in s.as_bytes().iter().map(|v| -> bool { *v == b'1' }).rev() {
        if is_one_flag {
            if digit_flag && cum + digit_num <= k {
                ans_len += 1;
                cum += digit_num;
                if (digit_num as i64) << 1 <= k as i64 {
                    digit_num <<= 1;
                } else {
                    digit_flag = false;
                }
            }
        } else {
            ans_len += 1;
            if (digit_num as i64) << 1 <= k as i64 {
                digit_num <<= 1;
            } else {
                digit_flag = false;
            }
        }
        // println!("{} {}", cum, digit_num);
    }
    ans_len
}
