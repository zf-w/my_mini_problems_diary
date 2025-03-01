//! ## Leetcode 950. Reveal Cards In Increasing Order
//! https://leetcode.com/problems/reveal-cards-in-increasing-order
//! - `Medium`; `Hinted by Tag`; `2024-04-09`;
//!
//! I wonder whether there are math solutions. I saw the tag simulation and found out that the simulation is just linear in time with the length of the input.

use std::collections::VecDeque;

pub fn deck_revealed_increasing(mut deck: Vec<i32>) -> Vec<i32> {
    let len = deck.len();
    let mut target: Vec<usize> = vec![0; len];
    let mut dq: VecDeque<usize> = VecDeque::with_capacity(len);
    for i in 0..len {
        dq.push_back(i);
    }
    for i in 0..len {
        if dq.len() > 1 {
            target[dq.pop_front().expect("Checked length")] = i;
            let to_back = dq.pop_front().expect("Checked length");
            dq.push_back(to_back);
        } else {
            target[dq.pop_front().expect("Checked length")] = i;
        }
    }
    let mut ans: Vec<i32> = Vec::with_capacity(len);
    deck.sort();
    for i in target {
        ans.push(deck[i]);
    }
    ans
}
