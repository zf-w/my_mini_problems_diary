//! ## Leetcode 2092. Find All People With Secret
//! https://leetcode.com/problems/find-all-people-with-secret
//! -`Hard`; `Hinted by Solution`; `2024-02-25`;
//!
//! One key problem of this problem is that you can't perform the BFS optimization that marks the node at enqueue time. An edge later might be actually earlier. That's the key to Dijkastra, I guess.

use std::collections::BinaryHeap;

pub fn find_all_people(n: i32, meetings: Vec<Vec<i32>>, first_person: i32) -> Vec<i32> {
    let n_usize = n as usize;
    let meetings_num = meetings.len();
    let mut pq: BinaryHeap<(i32, usize)> = BinaryHeap::with_capacity(meetings_num);

    let mut graph: Vec<Vec<(i32, usize)>> = vec![Vec::new(); n_usize];
    for meeting in meetings {
        let (time, a, b) = (meeting[2], meeting[0] as usize, meeting[1] as usize);
        graph[a].push((time, b));
        graph[b].push((time, a));
    }
    pq.push((0, 0));
    pq.push((0, first_person as usize));
    let mut ans = Vec::new();
    let mut visited: Vec<bool> = vec![false; n_usize];

    while let Some((curr_time, person)) = pq.pop() {
        if visited[person] {
            continue;
        }
        ans.push(person as i32);
        visited[person] = true;
        graph[person].sort();
        for (meet_time, next) in graph[person].iter() {
            if *meet_time >= -curr_time {
                pq.push((-*meet_time, *next));
            }
        }
    }

    ans
}
