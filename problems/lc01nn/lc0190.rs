//! # Leetcode 190. Reverse Bits
//! https://leetcode.com/problems/reverse-bits/
//! - y2026m02d16; Independently Solved;

pub fn reverse_bits(mut n: i32) -> i32 {
    let mut ans_num = 0;
    for _ in 0..32 {
        ans_num <<= 1;
        ans_num |= (n & 1);
        n >>= 1;
    }

    ans_num
}