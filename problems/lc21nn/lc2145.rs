//! # Leetcode 2145. Count the Hidden Sequences
//! https://leetcode.com/problems/count-the-hidden-sequences/
//! - `Medium`; `y2025m04d21`; `Independently Solved`; `0ms`; `3.8mb`; `1 attempt`;
//! Topics: uncategorized.

pub fn number_of_arrays(differences: Vec<i32>, lower: i32, upper: i32) -> i32 {
    let mut min_val: i64 = 0;
    let mut max_val: i64 = 0;
    let mut curr_val: i64 = 0;

    for diff in differences {
        curr_val += diff as i64;
        min_val = min_val.min(curr_val);
        max_val = max_val.max(curr_val);
    }

    let range = max_val - min_val;
    let bound_range = (upper - lower) as i64;

    (bound_range - range + 1).max(0) as i32
}
