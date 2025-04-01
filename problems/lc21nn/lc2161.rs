//! # Leetcode 2161. Partition Array According to Given Pivot
//! https://leetcode.com/problems/partition-array-according-to-given-pivot/
//! - `Medium`; `y2025m03d03`; `Independently Solved`; `0ms`; `3.9mb`; `1 attempt`;
//! Topics: uncategorized.

pub fn pivot_array(nums: Vec<i32>, pivot: i32) -> Vec<i32> {
    let mut ans_vec: Vec<i32> = Vec::with_capacity(nums.len());
    for num in nums.iter().cloned() {
        if num < pivot {
            ans_vec.push(num);
        }
    }
    for num in nums.iter().cloned() {
        if num == pivot {
            ans_vec.push(num);
        }
    }
    for num in nums.iter().cloned() {
        if num > pivot {
            ans_vec.push(num);
        }
    }
    ans_vec
}
