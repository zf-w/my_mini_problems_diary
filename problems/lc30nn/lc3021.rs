//! # Leetcode 3021. Alice and Bob Playing Flower Game
//! https://leetcode.com/problems/alice-and-bob-playing-flower-game/
//! - `Medium`; `y2025m08d28`; `Independently Solved`; `0ms`; `2.2mb`; `1 attempt`;
//! Topics: observe_patterns.

pub fn flower_game(n: i32, m: i32) -> i64 {
    let n = n as i64;
    let m = m as i64;
    let n_odd_num = (n / 2 + if n & 1 == 1 { 1 } else { 0 });
    let n_even_num = (n - n_odd_num);
    let m_odd_num = (m / 2 + if m & 1 == 1 { 1 } else { 0 });
    let m_even_num = m - m_odd_num;

    n_odd_num * m_even_num + n_even_num * m_odd_num
}
