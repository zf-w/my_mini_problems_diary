//! ## Leetcode 945. Minimum Increment to Make Array Unique
//! https://leetcode.com/problems/minimum-increment-to-make-array-unique/
//! - `Medium`; `Independently Solved`; `2024-06-19`;
//!
//! I guess the question will be more difficult if it allows us to decrease the `num[i]`. Currently, we can sort the numbers and see how many steps one number needs to move.

pub fn min_increment_for_unique(mut nums: Vec<i32>) -> i32 {
    nums.sort_unstable();
    let mut nums_iter = nums.iter().cloned();
    let mut prev = if let Some(v) = nums_iter.next() {
        v
    } else {
        return 0;
    };
    let mut ans_count = 0;
    for curr in nums_iter {
        if curr == prev {
            ans_count += 1;
            prev += 1;
        } else {
            prev = curr;
        }
    }
    ans_count
}
