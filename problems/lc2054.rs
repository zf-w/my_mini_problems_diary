//! # Leetcode 2054. Two Best Non-Overlapping Events
//! https://leetcode.com/problems/two-best-non-overlapping-events/
//! - `Medium`; `y2024m12d08`; `Learned from Solution`; `16ms`; `11.2mb`; `1 attempt`;
//!
//! Busy: learned from https://leetcode.com/problems/two-best-non-overlapping-events/solutions/6124257/sort-greedy-vs-dp-binary-search-22ms-beats-100

pub fn max_two_events(events: Vec<Vec<i32>>) -> i32 {
    let len = events.len();
    let mut event_info_vec: Vec<(i32, bool, i32)> = Vec::with_capacity(len * 2);
    for (i, event) in events.into_iter().enumerate() {
        let start_time = event[0];
        let end_time = event[1];
        let value = event[2];
        event_info_vec.push((start_time, false, value));
        event_info_vec.push((end_time, true, value));
    }

    event_info_vec.sort_unstable();

    let mut ans_value = 0;
    let mut curr_max = 0;
    for (_, curr_is_end_flag, curr_value) in event_info_vec {
        if curr_is_end_flag {
            curr_max = curr_max.max(curr_value);
        } else {
            ans_value = ans_value.max(curr_max + curr_value);
        }
    }
    ans_value
}
