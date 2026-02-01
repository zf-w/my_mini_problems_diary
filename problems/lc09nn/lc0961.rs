//! # Leetcode 961. N-Repeated Element in Size 2 N Array
//! https://leetcode.com/problems/n-repeated-element-in-size-2n-array/
//! - `Easy`; `y2026m01d02`; `Independently Solved`; `0ms`; `2.3mb`; `1 attempt`;

pub fn repeated_n_times(nums: Vec<i32>) -> i32 {
    use std::collections::HashSet;

    let mut num_set: HashSet<i32> = HashSet::with_capacity(nums.len());

    for num in nums {
        if num_set.contains(&num) {
            return num;
        }

        num_set.insert(num);
    }

    unreachable!()
}