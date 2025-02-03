//! # Leetcode 2558. Take Gifts From the Richest Pile
//! https://leetcode.com/problems/take-gifts-from-the-richest-pile/
//! - `Easy`; `y2024m12d12`; `Independently Solved`; `0ms`; `2.3mb`; `1 attempt`;

pub fn pick_gifts(gifts: Vec<i32>, k: i32) -> i64 {
    let mut heap: std::collections::BinaryHeap<i32> =
        std::collections::BinaryHeap::from_iter(gifts);

    for _ in 0..k {
        let top_pile = heap.pop().expect("gifts.len() > 0");
        heap.push((top_pile as f64).sqrt() as i32);
    }

    heap.into_iter().map(|v| -> i64 { v as i64 }).sum()
}
