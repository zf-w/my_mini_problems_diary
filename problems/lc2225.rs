//! ## Leetcode 2225. Find Players With Zero or One Losses
//! https://leetcode.com/problems/find-players-with-zero-or-one-losses
//! - `Medium`; `Independently Solved`; `2024-01-14`;
//!
//! Instead of counting the number of loses for all players, we don't need to track the excat number of loses for those already lost two matches.

use std::collections::HashSet;

pub fn find_winners(matches: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut lose0: HashSet<i32> = HashSet::new();
    let mut lose1: HashSet<i32> = HashSet::new();
    let mut lose: HashSet<i32> = HashSet::new();
    for m in matches.iter() {
        let a = m.get(0).expect("Have two");
        let b = m.get(1).expect("Have two");
        if !lose.contains(a) && !lose1.contains(a) && !lose0.contains(a) {
            lose0.insert(*a);
        }
        if lose1.contains(b) {
            lose1.remove(b);
            lose.insert(*b);
        } else if lose0.contains(b) {
            lose0.remove(b);
            lose1.insert(*b);
        } else if !lose.contains(b) {
            lose1.insert(*b);
        }
    }
    let mut l0: Vec<i32> = Vec::with_capacity(lose0.len());
    let mut l1: Vec<i32> = Vec::with_capacity(lose1.len());
    for v in lose0.iter() {
        l0.push(*v);
    }
    for v in lose1.iter() {
        l1.push(*v);
    }
    l0.sort();
    l1.sort();
    vec![l0, l1]
}
