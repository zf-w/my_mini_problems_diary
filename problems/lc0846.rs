//! ## Leetcode 846. Hand of Straights
//! https://leetcode.com/problems/hand-of-straights/
//! - `Medium`; `Learned from Solution`; `2024-06-06`;
//!
//! My first idea was to use an array to keep track of the counts of each number and then simulate the process. However, I was worried that the space would be too big.
//!
//! I scanned through the solution: https://leetcode.com/problems/divide-array-in-sets-of-k-consecutive-numbers/solutions/5267524/simple-and-easy-c-solution. I feel it might be a good chance to practice with [Entry] of [BTreeMap]. We can simulate the process using the [BTreeMap]: finding the first entry and checking consecutive numbers.
//!

use std::collections::{
    btree_map::{Entry, OccupiedEntry},
    BTreeMap,
};

pub fn is_n_straight_hand(hand: Vec<i32>, group_size: i32) -> bool {
    if hand.len() % group_size as usize > 0 {
        return false;
    }
    let mut mp: BTreeMap<i32, usize> = BTreeMap::new();
    for num in hand.iter().cloned() {
        mp.entry(num).and_modify(|curr| *curr += 1).or_insert(1);
    }
    #[inline]
    fn handle_entry(mut curr: OccupiedEntry<i32, usize>) -> Option<i32> {
        let curr_key = curr.key().clone();
        let curr_val_mut_ref = curr.get_mut();
        if *curr_val_mut_ref > 0 {
            *curr_val_mut_ref -= 1;
            if *curr_val_mut_ref == 0 {
                curr.remove_entry();
            }
            Some(curr_key)
        } else {
            None
        }
    }
    while let Some(mut curr) = mp.first_entry() {
        for _ in 0..(group_size as usize - 1) {
            let curr_key = if let Some(curr_key) = handle_entry(curr) {
                curr_key
            } else {
                return false;
            };

            curr = if let Entry::Occupied(next) = mp.entry(curr_key + 1) {
                next
            } else {
                return false;
            }
        }
        if handle_entry(curr).is_none() {
            return false;
        }
    }

    true
}
