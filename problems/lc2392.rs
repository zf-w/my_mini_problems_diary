//! ## Leetcode 2392. Build a Matrix With Conditions
//! https://leetcode.com/problems/build-a-matrix-with-conditions/
//! - `Medium`; `Hinted`; `2024-07-21`;

use std::collections::VecDeque;

fn sort_numbers_based_on_conditions(len: usize, conditions: Vec<Vec<i32>>) -> Option<Vec<usize>> {
    let mut adj: Vec<(Vec<usize>, usize)> = vec![(Vec::new(), 0); len];
    for condition in conditions.iter() {
        let first_i = (condition[1] - 1) as usize;
        let second_i = (condition[0] - 1) as usize;
        let first_node_mut_ref = adj
            .get_mut(first_i)
            .expect("getting first node info mut ref");
        first_node_mut_ref.1 += 1;
        let second_node_mut_ref = adj
            .get_mut(second_i)
            .expect("getting second node info mut ref");
        second_node_mut_ref.0.push(first_i);
    }
    let mut wait_dq: VecDeque<usize> = VecDeque::with_capacity(len);
    for (curr_i, (_, prereq_count_ref)) in adj.iter().enumerate() {
        if *prereq_count_ref == 0 {
            wait_dq.push_back(curr_i);
        }
    }

    let mut ans_vec: Vec<usize> = vec![0; len];
    let mut count: usize = 0;

    while let Some(curr_i) = wait_dq.pop_front() {
        ans_vec[curr_i] = count;
        count += 1;
        for next_i_i in 0..adj[curr_i].0.len() {
            let next_i = adj[curr_i].0[next_i_i];
            let next_prereq_mut_ref = &mut adj.get_mut(next_i).expect("getting next").1;
            *next_prereq_mut_ref -= 1;
            if adj[next_i].1 == 0 {
                wait_dq.push_back(next_i);
            }
        }
    }

    if count < len {
        return None;
    }

    Some(ans_vec)
}

pub fn build_matrix(
    k: i32,
    row_conditions: Vec<Vec<i32>>,
    col_conditions: Vec<Vec<i32>>,
) -> Vec<Vec<i32>> {
    let len = k as usize;
    let row_idxs = if let Some(row_idxs) = sort_numbers_based_on_conditions(len, row_conditions) {
        row_idxs
    } else {
        return Vec::new();
    };
    let col_idxs = if let Some(col_idxs) = sort_numbers_based_on_conditions(len, col_conditions) {
        col_idxs
    } else {
        return Vec::new();
    };
    let mut ans_matrix: Vec<Vec<i32>> = vec![vec![0; len]; len];
    for (curr_i, (row_i, col_i)) in row_idxs
        .iter()
        .cloned()
        .zip(col_idxs.iter().cloned())
        .enumerate()
    {
        ans_matrix[row_i][col_i] = curr_i as i32 + 1;
    }
    ans_matrix
}
