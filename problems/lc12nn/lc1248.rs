//! ## Leetcode 1248. Count Number of Nice Subarrays
//! https://leetcode.com/problems/count-number-of-nice-subarrays/
//! - `Medium`; `Independently Solved`; `2024-06-21`;
//!
//! We can solve this using the `Sliding Window` idea. A new number appended to the end of the array will increase the answer number by the distance between the `k`th odd number and the `k + 1`th odd number (or one before the beginning of the array).

use std::collections::VecDeque;

pub fn number_of_subarrays(nums: Vec<i32>, k: i32) -> i32 {
    let k = k as usize;
    let nums_odd_iter = nums.iter().map(|v| -> bool { (v & 1) == 1 });
    let mut begin_i = 0;
    let mut odd_iter = nums_odd_iter.enumerate();

    let mut deque: VecDeque<usize> = VecDeque::new();
    let mut ans_count = 0;
    while let Some((i, is_odd)) = odd_iter.next() {
        if is_odd {
            deque.push_back(i);
        }
        let mut curr_len = deque.len();
        if curr_len > k {
            let begin_odd_i = deque.pop_front().expect("k > 0");
            begin_i = begin_odd_i + 1;
            curr_len -= 1;
        }
        if curr_len == k {
            let begin_odd_i = deque.front().expect("k > 0");
            ans_count += begin_odd_i - begin_i + 1;
            // println!("{},{}", begin_i, begin_odd_i);
        }
    }
    ans_count as i32
}
