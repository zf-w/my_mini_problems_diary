//! ## Leetcode 729. My Calendar I
//! https://leetcode.com/problems/my-calendar-i/
//! - `Medium`; `Independently Solved`; `y2024m09d25`;

use std::collections::BTreeSet;
struct MyCalendar {
    intervals_map: BTreeSet<(i32, i32)>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCalendar {
    fn new() -> Self {
        Self {
            intervals_map: BTreeSet::new(),
        }
    }

    fn book(&mut self, start: i32, end: i32) -> bool {
        if let Some((_, prev_end_ref)) = self.intervals_map.range(..(start, end)).last() {
            if *prev_end_ref > start {
                return false;
            }
        }
        if self
            .intervals_map
            .range((start, start)..(end, end))
            .next()
            .is_some()
            == true
        {
            return false;
        }
        // if self.intervals_map.range((end, end)..).next().is_some() == true {
        //     return false;
        // }
        self.intervals_map.insert((start, end));
        true
    }
}
