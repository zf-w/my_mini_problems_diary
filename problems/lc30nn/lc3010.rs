//! # Leetcode 3010. Divide an Array Into Subarrays With Minimum Cost I
//! https://leetcode.com/problems/divide-an-array-into-subarrays-with-minimum-cost-i/
//! - y2026m02d01; Independently Solved

pub fn minimum_cost(nums: Vec<i32>) -> i32 {
        use std::collections::BinaryHeap;

    let mut num_iter = nums.into_iter();
    let mut ans_sum = num_iter.next().expect("len > 0");

    let mut max_two_heap: BinaryHeap<i32> = BinaryHeap::with_capacity(3);

    for num in num_iter {
        max_two_heap.push(num);
        if max_two_heap.len() > 2 {
            max_two_heap.pop();
        }
    }

    for num in max_two_heap.iter() {
        ans_sum += *num;
    }

    ans_sum
}