//! ## Leetcode 752. Open the Lock
//! https://leetcode.com/problems/open-the-lock
//! - `Medium`; `Independently Solved`; `2024-04-22`;
//!s
//! An interesting BFS problem. How to efficiently and naturally represent the numbers on the current lock is interesting to think about.

use std::collections::{HashSet, VecDeque};
type Wheel = [u8; 4];
pub fn open_lock(deadends: Vec<String>, target: String) -> i32 {
    let mut visited: HashSet<Wheel> = HashSet::new();
    fn string_to_wheel(s: &String) -> Wheel {
        let base = '0' as u8;
        let b = s.as_bytes();
        [
            b[0] as u8 - base,
            b[1] as u8 - base,
            b[2] as u8 - base,
            b[3] as u8 - base,
        ]
    }
    let target_w = string_to_wheel(&target);
    if target_w == [0, 0, 0, 0] {
        return 0;
    }
    let deadends_w: HashSet<Wheel> = HashSet::from_iter(deadends.iter().map(string_to_wheel));
    let mut q: VecDeque<Wheel> = VecDeque::new();
    q.push_back([0, 0, 0, 0]);
    let mut dis = 0;
    while !q.is_empty() {
        let curr_q_len = q.len();
        for _ in 0..curr_q_len {
            let curr_w = q.pop_front().expect("Checked size");
            if curr_w == target_w {
                return dis;
            }
            if deadends_w.contains(&curr_w) {
                continue;
            }
            for pos in 0..4usize {
                let mut next0 = curr_w;
                next0[pos] = (next0[pos] + 9) % 10;
                let mut next1 = curr_w;
                next1[pos] = (next1[pos] + 1) % 10;
                if !visited.contains(&next0) {
                    visited.insert(next0.clone());
                    q.push_back(next0);
                }
                if !visited.contains(&next1) {
                    visited.insert(next1.clone());
                    q.push_back(next1);
                }
            }
        }
        dis += 1;
    }
    -1
}
