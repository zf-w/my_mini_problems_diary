//! # Leetcode 2551. Put Marbles in Bags
//! https://leetcode.com/problems/put-marbles-in-bags/
//! - `Hard`; `y2025m03d31`; `Learned from Solution`; `5ms`; `3.6mb`; `1 attempt`;
//! Topics: uncategorized.
//!
//!
//! Learned from https://leetcode.com/problems/put-marbles-in-bags/solutions/3260382/put-marbles-in-bags.

pub fn put_marbles(mut weights: Vec<i32>, k: i32) -> i64 {
    let len = weights.len();
    for i in 1..len {
        let prev_i = i - 1;
        weights[prev_i] += weights[i];
    }

    weights.pop();

    weights.sort_unstable();

    let mut ans: i64 = 0;
    let k = k as usize;

    let pair_weight_vec = weights;

    for i in 0..(k - 1) {
        ans += (pair_weight_vec[len - 2 - i] - pair_weight_vec[i]) as i64;
    }

    ans
}
