//! # Leetcode 2529. Maximum Count of Positive Integer and Negative Integer
//! https://leetcode.com/problems/maximum-count-of-positive-integer-and-negative-integer/
//! - `Easy`; `y2025m03d11`; `Independently Solved`; `0ms`; `2.3mb`; `1 attempt`;
//! Topics: uncategorized.

pub fn maximum_count(nums: Vec<i32>) -> i32 {
    let (pos_count, neg_count) =
        nums.into_iter()
            .fold((0, 0), |(pos_count, neg_count), v| -> (i32, i32) {
                if v > 0 {
                    (pos_count + 1, neg_count)
                } else if v < 0 {
                    (pos_count, neg_count + 1)
                } else {
                    (pos_count, neg_count)
                }
            });
    pos_count.max(neg_count)
}
