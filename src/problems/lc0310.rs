//! ## Leetcode 310. Minimum Height Trees
//! https://leetcode.com/problems/minimum-height-trees
//! - `Medium`; `Learned from Solution`; `2024-04-22`;
//!
//! Getting the sense but not fully understanding the idea. I spent most of the time debugging why I couldn't push `Vec's with capacities on line 16 with the last few data points.

use std::collections::VecDeque;

pub fn find_min_height_trees(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
    if n == 1 {
        return vec![0];
    }
    let n_usize = n as usize;
    let mut adj_vec: Vec<Vec<usize>> = Vec::with_capacity(n_usize);
    for _ in 0..n_usize {
        adj_vec.push(Vec::new());
    }
    let mut degs: Vec<usize> = vec![0; n_usize];
    for edge in edges.iter() {
        let i = edge[0] as usize;
        let j = edge[1] as usize;
        adj_vec[i].push(j);
        adj_vec[j].push(i);
        degs[i] += 1;
        degs[j] += 1;
    }

    let mut q: VecDeque<usize> = VecDeque::with_capacity(n_usize);
    let mut left = n_usize;
    for (i, nexts) in adj_vec.iter().enumerate() {
        if nexts.len() == 1 {
            q.push_back(i);
            left -= 1;
        }
    }

    while left > 0 && !q.is_empty() {
        let curr_q_len = q.len();

        for _ in 0..curr_q_len {
            let curr = q.pop_front().expect("Checked len");

            for next in adj_vec[curr].iter() {
                degs[*next] -= 1;

                if degs[*next] == 1 {
                    q.push_back(*next);
                }
            }
        }
        left -= q.len();
    }

    Vec::from_iter(q.iter().map(|v| -> i32 { (*v) as i32 }))
}
