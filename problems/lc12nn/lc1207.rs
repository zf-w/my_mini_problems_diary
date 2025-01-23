//! ## Leetcode 1207. Unique Number of Occurrences
//! https://leetcode.com/problems/unique-number-of-occurrences
//! - `Easy`; `Independently Solved`; `2024-01-16`;
//!
//! I guess using "filter" to replace "if continue" would be a more natural way.

use std::collections::HashSet;

pub fn unique_occurrences(arr: Vec<i32>) -> bool {
    let mut count: [usize; 2001] = [0; 2001];
    fn index(v: i32) -> usize {
        (v + 1000) as usize
    }

    for num in arr.iter() {
        count[index(*num)] += 1;
    }
    let mut s: HashSet<usize> = HashSet::new();
    for v in count.iter().filter(|v| **v > 0) {
        let good = s.insert(*v);
        if !good {
            return false;
        }
    }
    true
}
