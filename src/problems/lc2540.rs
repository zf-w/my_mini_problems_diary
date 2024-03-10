//! ## Leetcode 2540. Minimum Common Value
//! https://leetcode.com/problems/minimum-common-value
//! - `Easy`; `Independetnly Solved`; `2024-03-09`;
//!
//! Ops. I didn't see that the two arrays are sorted in non-decreasing order.

use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
};

pub fn get_common(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
    let mut set_1: HashSet<i32> = HashSet::with_capacity(nums1.len() * 2);
    for num in nums1 {
        set_1.insert(num);
    }
    let mut pq_2: BinaryHeap<Reverse<i32>> =
        BinaryHeap::from_iter(nums2.iter().map(|v| -> Reverse<i32> { Reverse(*v) }));

    while let Some(curr) = pq_2.pop() {
        if set_1.contains(&curr.0) {
            return curr.0;
        }
    }
    return -1;
}
