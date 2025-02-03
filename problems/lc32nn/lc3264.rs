//! # Leetcode 3264. Final Array State After K Multiplication Operations I
//! https://leetcode.com/problems/final-array-state-after-k-multiplication-operations-i/
//! - `Easy`; `y2024m12d16`; `Independently Solved`; `0ms`; `2.3mb`; `1 attempt`;

pub fn get_final_state(mut nums: Vec<i32>, k: i32, multiplier: i32) -> Vec<i32> {
    use std::collections::BinaryHeap;
    let mut heap: BinaryHeap<(i32, usize)> = BinaryHeap::with_capacity(nums.len());
    for (i, num) in nums.iter().cloned().enumerate() {
        heap.push((-num, i));
    }

    for _ in 0..k {
        let mut popped_elem = heap.pop().expect("Not removing");
        popped_elem.0 *= multiplier;
        heap.push(popped_elem);
    }

    for (val, i) in heap {
        nums[i] = -val;
    }
    nums
}
