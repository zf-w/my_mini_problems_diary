//! # Leetcode 3066. Minimum Operations to Exceed Threshold Value II
//! https://leetcode.com/problems/minimum-operations-to-exceed-threshold-value-ii/
//! - `Medium`; `y2025m02d13`; `Independently Solved`; `31ms`; `5.4mb`; `2 attempts`;

pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
    use std::collections::BinaryHeap;
    let k = k as i64; // First bug here, i32 not large enough.
    let mut num_pq: BinaryHeap<i64> =
        BinaryHeap::from_iter(nums.into_iter().map(|v| -> i64 { (-v) as i64 }));
    let mut ans_count = 0;
    while let Some(min_num) = num_pq.pop() {
        let min_num = -min_num;
        // println!("{}", min_num);
        if min_num >= k {
            return ans_count;
        }

        let next_num = -num_pq.pop().expect("Input guaranteed.");

        num_pq.push(-(next_num.min(min_num) * 2 + next_num.max(min_num)));

        ans_count += 1;
    }

    ans_count
}
