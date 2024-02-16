//! ## Leetcode 1481. Least Number of Unique Integers after K Removals
//! https://leetcode.com/problems/least-number-of-unique-integers-after-k-removals
//! - `Medium`; `Independently Solved`; `2024-02-15`;
//!
//! Read through the instructions for BinaryHeap and find out the `Reverse`. The `Reverse`'s pretty helpful.

use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};
pub fn find_least_num_of_unique_ints(arr: Vec<i32>, k: i32) -> i32 {
    let len = arr.len();
    let mut counts: HashMap<i32, usize> = HashMap::with_capacity(len * 2);
    for num in arr {
        if let Some(count) = counts.get_mut(&num) {
            *count += 1;
        } else {
            counts.insert(num, 1);
        }
    }
    let mut left = counts.len();
    let mut hp: BinaryHeap<Reverse<usize>> = BinaryHeap::with_capacity(left);
    for (_, count) in counts.iter() {
        hp.push(Reverse(*count));
    }

    let mut k_usize = k as usize;
    while let Some(Reverse(curr)) = hp.pop() {
        if curr <= k_usize {
            left -= 1;
            k_usize -= curr;
        } else {
            break;
        }
    }
    left as i32
}
