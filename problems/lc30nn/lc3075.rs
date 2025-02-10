//! ## Leetcode 3075. Maximize Happiness of Selected Children
//! https://leetcode.com/problems/maximize-happiness-of-selected-children
//! - `Medium`; `Learned from Solution`; `2024-05-8`
//!
//! This problem involves selecting the top k elements and summating them under some special conditions. It's also possible to use the partition idea from quick sort to get the top k elements. However, due to the special conditions of this problem, we still need to sort the top k elements.

use std::collections::BinaryHeap;

pub fn maximum_happiness_sum(happiness: Vec<i32>, mut k: i32) -> i64 {
    let len = happiness.len();
    let mut pq: BinaryHeap<i32> = BinaryHeap::with_capacity(len);

    for v in happiness {
        pq.push(v);
    }

    let mut ans_sum: i64 = 0;
    let mut count = 0;
    while k >= 0 {
        let top_v: i32 = pq.pop().expect("Should have") - count;
        if top_v > 0 {
            ans_sum += top_v as i64;
        } else {
            break;
        }
        count += 1;
        k -= 1;
    }
    ans_sum
}
