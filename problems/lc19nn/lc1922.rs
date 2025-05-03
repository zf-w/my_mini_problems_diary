//! # Leetcode 1922. Count Good Numbers
//! https://leetcode.com/problems/count-good-numbers/
//! - `Medium`; `y2025m04d12`; `Independently Solved`; `0ms`; `2.3mb`; `1 attempt`;
//! Topics: quick_power;

pub fn count_good_numbers(n: i64) -> i32 {
    const MODULO: i64 = 1000000007;
    fn quick_mod_power(mut n: i64, mut p: i64) -> i64 {
        let mut ans: i64 = 1;
        while p > 0 {
            if p & 1 == 1 {
                ans = (ans * n) % MODULO;
            }
            n = (n * n) % MODULO;
            p >>= 1;
        }

        ans
    }

    (quick_mod_power(5, (n + 1) / 2) * quick_mod_power(4, n / 2) % MODULO) as i32
}
