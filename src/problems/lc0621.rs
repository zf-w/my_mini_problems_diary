//! ## Leetcode 621. Task Scheduler
//! https://leetcode.com/problems/task-scheduler
//! - "Medium"; `Independently Solved`; `2023-03-18`;
//!
//! At first, I was thinking about using a priority queue to pop out the earliest ready-to-execute task. I found that didn't work because the essence of this problem would be to think of a way to lift the most frequent task to execute first.
//!
//! After a second thought, I think maybe grouping tasks together and prioritizing the most frequent task first would work. Yes, it indeed worked. I found the others' solution was only about counting the most frequent task and how many tasks have that frequency, since, whatever tasks we have, we always need "n + 1" times the frequency of it plus grouping the remaining ones with that frequency as the last group.

use std::collections::BinaryHeap;

pub fn least_interval(tasks: Vec<char>, n: i32) -> i32 {
    const TASK_TYPES_NUM: usize = 26;
    let mut counts: [usize; TASK_TYPES_NUM] = [0; TASK_TYPES_NUM];

    let char_base: usize = 'A' as usize;

    let char_to_idx = |c: &char| -> usize { *c as usize - char_base };
    for task in tasks.iter() {
        counts[char_to_idx(task)] += 1;
    }

    let mut ans = 0;
    let mut init: Vec<(usize, usize)> = Vec::with_capacity(TASK_TYPES_NUM);
    for i in 0..TASK_TYPES_NUM {
        if counts[i] > 0 {
            init.push((counts[i], i));
        }
    }
    let mut pq: BinaryHeap<(usize, usize)> = BinaryHeap::from(init);
    let group_size = 1 + n as usize;

    let mut wait: Vec<(usize, usize)> = Vec::with_capacity(TASK_TYPES_NUM);
    while !pq.is_empty() {
        let mut curr_used: i32 = 0;

        for _ in 0..group_size {
            if pq.is_empty() {
                break;
            }
            let mut curr = pq.pop().unwrap();
            curr.0 -= 1;
            // print!("{} ", curr.1);
            if curr.0 > 0 {
                wait.push(curr);
            }
            curr_used += 1;
        }
        if !wait.is_empty() {
            ans += n + 1;
            // println!("{}", ans);
            while let Some(curr) = wait.pop() {
                pq.push(curr);
            }
        } else {
            ans += curr_used;
            // println!("{}", ans);
        }
    }
    ans
}

#[test]
fn case_0() {
    let tasks = vec!['A', 'A', 'A', 'B', 'B', 'B'];
    assert_eq!(8, least_interval(tasks, 2));
}

#[test]
fn case_1() {
    let tasks = vec!['A', 'C', 'A', 'B', 'D', 'B'];
    assert_eq!(6, least_interval(tasks, 1));
}

#[test]
fn case_2() {
    let tasks = vec!['A', 'A', 'A', 'B', 'B', 'B'];
    assert_eq!(10, least_interval(tasks, 3));
}

#[test]
fn case_3() {
    let tasks = vec!['A', 'A', 'A', 'A', 'A', 'A', 'B', 'C', 'D', 'E', 'F', 'G'];
    assert_eq!(12, least_interval(tasks, 1));
}
