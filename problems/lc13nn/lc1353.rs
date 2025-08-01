//! #  Leetcode 1353. Maximum Number of Events That Can Be Attended
//! https://leetcode.com/problems/maximum-number-of-events-that-can-be-attended/
//! - `Medium`; `y2025m07d07`; `Learned from Solution`; `24ms`; `10.5mb`; `2 attempts`;
//! Topics: greedy.

pub fn max_events(mut events: Vec<Vec<i32>>) -> i32 {
    use std::cmp::Ordering;

    let event_vec_len = events.len();
    events.sort_unstable_by(|a_ref: &Vec<i32>, b_ref: &Vec<i32>| -> std::cmp::Ordering {
        let a_begin_day_i = a_ref[0];
        let a_end_day_i = a_ref[1];
        let b_begin_i = b_ref[0];
        let b_end_day_i = b_ref[1];
        if a_begin_day_i < b_begin_i || (a_begin_day_i == b_begin_i && a_end_day_i < b_end_day_i) {
            Ordering::Less
        } else if a_begin_day_i == b_begin_i && a_end_day_i == b_end_day_i {
            Ordering::Equal
        } else {
            Ordering::Greater
        }
    });
    use std::collections::BinaryHeap;

    let mut pq: BinaryHeap<i32> = BinaryHeap::with_capacity(event_vec_len);

    let mut ans = 0;

    let mut event_i = 0;
    let mut day_i = 0;
    while event_i < event_vec_len || pq.is_empty() == false {
        // println!("{day_i}");
        while event_i < event_vec_len && events[event_i][0] <= day_i {
            // println!("add end {}",events[event_i][1]);
            pq.push(-events[event_i][1]);
            event_i += 1;
        }

        while let Some(end_day_i) = pq.pop() {
            // println!("pop end {}", end_day_i);
            if -end_day_i >= day_i {
                ans += 1;
                break;
            }
        }

        day_i += 1;
    }
    ans
}