//! ## Leetcode 2402. Meeting Rooms III
//! https://leetcode.com/problems/meeting-rooms-iii
//! - `Hard`; `Hinted`; `2024-02-17`;
//!
//! The final bug was about integer overflow... It took me at least an hour to find it, and I can tell that's a deliberate design. I would say it's fine, but still...

use std::{cmp::Reverse, collections::BinaryHeap};

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Note {
    end: u32,
    room: usize,
    id: u32,
}

pub fn most_booked(n: i32, mut meetings: Vec<Vec<i32>>) -> i32 {
    meetings.sort();
    let n_usize: usize = n as usize;
    let mut count: Vec<i32> = Vec::with_capacity(n_usize);
    let mut started: BinaryHeap<Reverse<Note>> = BinaryHeap::with_capacity(n_usize);
    let mut wait: BinaryHeap<Reverse<usize>> = BinaryHeap::with_capacity(n_usize);
    for i in 0..n_usize {
        count.push(0);
        wait.push(Reverse(i));
    }
    for (_, meeting) in meetings.iter().enumerate() {
        let curr_start = *meeting.get(0).expect("Should have start") as u32;
        let curr_end = *meeting.get(1).expect("Should have end") as u32;
        while let Some(Reverse(Note { end, id: _, room })) = started.peek() {
            if *end > curr_start {
                break;
            } else {
                wait.push(Reverse(*room));
                started.pop();
            }
        }
        if let Some(Reverse(room)) = wait.pop() {
            count[room] += 1;
            started.push(Reverse(Note {
                end: curr_end as u32,
                id: curr_start,
                room,
            }));
            // println!("{} -> {}", id, room);
        } else {
            if let Some(Reverse(Note { end, id: _, room })) = started.pop() {
                let delayed_end = if end <= curr_start {
                    curr_end
                } else {
                    end + curr_end - curr_start
                };
                started.push(Reverse(Note {
                    end: delayed_end as u32,
                    id: end,
                    room,
                }));
                count[room] += 1;
                // if room == 1 && id > 70000 {
                //     println!("{} -> {}, Start: {}, ETE: {}", id, room, end, delayed_end);
                // }
            }
        }
    }
    let mut iter = count.iter();
    let mut ans: i32 = 0;
    let mut count: i32 = *iter.next().expect("Should have one");
    for (i, curr) in iter.enumerate() {
        // println!("{}, {}", *curr, count);
        if *curr > count {
            count = *curr;
            ans = 1 + i as i32;
        }
    }
    ans
}
