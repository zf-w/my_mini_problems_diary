//! # Leetcode 2294. Partition Array Such That Maximum Difference is K
//! https://leetcode.com/problems/partition-array-such-that-maximum-difference-is-k/
//! - `Medium`; `y2025m06d19`; `Independently Solved`; `7ms`; `3.7mb`; `1 attempt`;
//! Topics: sorting.

pub fn partition_array(mut nums: Vec<i32>, k: i32) -> i32 {
    nums.sort_unstable();

    let mut num_iter = nums.into_iter();

    let mut prev = num_iter.next().expect("len > 0");

    let mut ans_count = 0;

    for num in num_iter {
        if num - prev > k {
            ans_count += 1;
            prev = num;
        }
    }

    ans_count
}
