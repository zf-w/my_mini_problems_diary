//! # Leetcode 1014. Best Sightseeing Pair
//! https://leetcode.com/problems/best-sightseeing-pair/
//! - `Medium`; `y2024m12d27`; `Hinted`; `0ms`; 2.6mb`; `1 attempt`;

pub fn max_score_sightseeing_pair(values: Vec<i32>) -> i32 {
    let mut value_iter = values.into_iter();
    let mut prev_value = value_iter
        .next()
        .expect("values.len() > 1, or failing to get the first value.");
    prev_value -= 1;
    let mut ans_score = 0;
    for curr_value in value_iter {
        ans_score = ans_score.max(curr_value + prev_value);
        prev_value = prev_value.max(curr_value);
        prev_value -= 1;
    }
    ans_score
}
