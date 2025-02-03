//! # Leetcode 2493. Divide Nodes Into the Maximum Number of Groups
//! https://leetcode.com/problems/divide-nodes-into-the-maximum-number-of-groups/
//! - `Hard`; `y2025m02d03`; `Learned from Solution`; `39ms`; `2.8mb`; `2 attempts`;
//! BFS, Bipartisan.
//!
//! First cpp submission copied from https://leetcode.com/problems/divide-nodes-into-the-maximum-number-of-groups/
//! The below Rust Submission is my implementation.

use std::collections::VecDeque;

fn fn_is_bipartisan(
    adj_vec_arr_ref: &[Vec<usize>],
) -> Option<(usize, Vec<usize>, VecDeque<usize>)> {
    // Trying to reuse the allocated VecDeque...
    let node_num = adj_vec_arr_ref.len();

    let mut color_idx_vec: Vec<usize> = vec![node_num; node_num];
    let mut color_bool_vec: Vec<(bool, bool)> = vec![(false, false); node_num];
    let mut bfs_queue: VecDeque<usize> = VecDeque::with_capacity(node_num);

    let mut component_num: usize = 0;

    for node_i in 0..node_num {
        if color_idx_vec[node_i] < node_num {
            continue;
        }

        let curr_component_idx = component_num;
        color_idx_vec[node_i] = curr_component_idx;
        color_bool_vec[node_i] = (true, false);

        component_num += 1;

        bfs_queue.push_back(node_i);

        let mut expected_next_level_color = true;

        while bfs_queue.is_empty() == false {
            let level_num = bfs_queue.len();
            for _ in 0..level_num {
                let curr_node_i = bfs_queue.pop_front().expect("checked len");
                let curr_next_node_i_arr_ref = adj_vec_arr_ref[curr_node_i].as_slice();

                for next_node_i in curr_next_node_i_arr_ref.iter().cloned() {
                    let (next_is_set, next_color) = color_bool_vec[next_node_i];
                    if next_is_set && next_color != expected_next_level_color {
                        return None;
                    }

                    // Zeroth bug here, needing to check visited after checking bipartisan.
                    if color_idx_vec[next_node_i] == curr_component_idx {
                        continue;
                    }

                    color_bool_vec[next_node_i] = (true, expected_next_level_color);
                    color_idx_vec[next_node_i] = curr_component_idx;

                    bfs_queue.push_back(next_node_i);
                }
            }
            expected_next_level_color = !expected_next_level_color;
        }
        bfs_queue.clear();
    }

    Some((component_num, color_idx_vec, bfs_queue))
}

pub fn magnificent_sets(n: i32, edges: Vec<Vec<i32>>) -> i32 {
    let node_num: usize = n as usize;
    let mut adj_vec_vec: Vec<Vec<usize>> = vec![Vec::new(); node_num];

    for edge_vec in edges {
        let node_0_i = edge_vec[0] as usize - 1;
        let node_1_i = edge_vec[1] as usize - 1;

        adj_vec_vec[node_0_i].push(node_1_i);
        adj_vec_vec[node_1_i].push(node_0_i);
    }

    let (component_num, node_color_vec, mut bfs_queue) =
        if let Some(info) = fn_is_bipartisan(&adj_vec_vec) {
            info
        } else {
            return -1;
        };

    let mut component_max_dis_vec: Vec<usize> = vec![0; component_num];

    let mut visited_flag_vec: Vec<bool> = vec![false; node_num];

    for (node_i, curr_component_idx) in node_color_vec.iter().cloned().enumerate() {
        bfs_queue.push_back(node_i);

        let mut expected_next_level_color = true;

        let mut curr_dis: usize = 0;

        // Second bug here, forgetting to set visited to the first node to true.
        visited_flag_vec[node_i] = true;

        while bfs_queue.is_empty() == false {
            let level_num = bfs_queue.len();
            for _ in 0..level_num {
                let curr_node_i = bfs_queue.pop_front().expect("checked len");
                let curr_next_node_i_arr_ref = adj_vec_vec[curr_node_i].as_slice();

                for next_node_i in curr_next_node_i_arr_ref.iter().cloned() {
                    if visited_flag_vec[next_node_i] {
                        continue;
                    }
                    visited_flag_vec[next_node_i] = true;

                    bfs_queue.push_back(next_node_i);
                }
            }
            expected_next_level_color = !expected_next_level_color;
            curr_dis += 1;
        }

        component_max_dis_vec[curr_component_idx] =
            component_max_dis_vec[curr_component_idx].max(curr_dis);
        bfs_queue.clear();

        for entry_mut_ref in visited_flag_vec.iter_mut() {
            // Fisrt Bug here, forgetting to reset visited flags.
            *entry_mut_ref = false;
        }
    }

    component_max_dis_vec.into_iter().sum::<usize>() as i32
}
