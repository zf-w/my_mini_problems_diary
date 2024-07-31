//! ## Leetcode 2045. Second Minimum Time to Reach Destination
//! https://leetcode.com/problems/second-minimum-time-to-reach-destination/
//! - `Hard`; `Learned from Solution`; `2024-07-28`;
//!
//! Learn from https://leetcode.com/problems/second-minimum-time-to-reach-destination/solutions/5546176/bfs-dist-dist2-368ms-beats-97-27

use std::{collections::VecDeque, i32};

pub fn second_minimum(n: i32, edges: Vec<Vec<i32>>, time: i32, change: i32) -> i32 {
    let len = n as usize;
    let target = len - 1;
    let mut adj_vec: Vec<Vec<usize>> = vec![Vec::new(); len];
    for edge in edges {
        let node_0_i = edge[0] as usize - 1;
        let node_1_i = edge[1] as usize - 1;
        adj_vec[node_0_i].push(node_1_i);
        adj_vec[node_1_i].push(node_0_i);
    }

    let mut pq: VecDeque<(usize, i32)> = VecDeque::new();
    pq.push_back((0, 0));

    let mut dists_0: Vec<i32> = vec![i32::MAX; len];
    let mut dists_1: Vec<i32> = vec![i32::MAX; len];
    dists_0[0] = 0;

    while let Some((curr_node_i, dis)) = pq.pop_front() {
        for next_i in adj_vec
            .get(curr_node_i)
            .expect("Getting nexts")
            .iter()
            .cloned()
        {
            let next_dis = dis + 1;
            let next_dis_0_mut_ref = dists_0
                .get_mut(next_i)
                .expect("Updating dists 0 for curr next");
            let next_dis_1_mut_ref = dists_1
                .get_mut(next_i)
                .expect("Updating dists 0 for curr next");
            if next_dis < *next_dis_0_mut_ref {
                dists_1[next_i] = *next_dis_0_mut_ref;
                *next_dis_0_mut_ref = next_dis;
            } else if next_dis > *next_dis_0_mut_ref && next_dis < *next_dis_1_mut_ref {
                *next_dis_1_mut_ref = next_dis;
            } else {
                continue;
            }
            pq.push_back((next_i, next_dis));
        }
    }
    let step_num = dists_1[target];
    let mut ans_time = 0;
    for _ in 0..step_num {
        let interval_i = ans_time / change;
        if interval_i & 1 == 1 {
            ans_time = interval_i * change + change;
        }
        ans_time += time;
    }
    ans_time
}
