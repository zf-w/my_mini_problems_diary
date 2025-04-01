//! ## Leetcode 1509. Minimum Difference Between Largest and Smallest Value in Three Moves
//! https://leetcode.com/problems/minimum-difference-between-largest-and-smallest-value-in-three-moves/
//! - `Medium`; `Independently Solved`; `2024-07-03`;

pub fn min_difference(mut nums: Vec<i32>) -> i32 {
    let len = nums.len();
    if len <= 3 {
        return 0;
    }
    nums.sort();
    let skip_len = len - 3;
    let mut ans_min_val = i32::MAX;
    for i in 0..3usize {
        ans_min_val = ans_min_val.min(nums[i + skip_len] - nums[i]);
    }
    ans_min_val
}
