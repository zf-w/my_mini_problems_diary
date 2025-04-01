//! # Leetcode 2685. Count the Number of Complete Components
//! https://leetcode.com/problems/count-the-number-of-complete-components/
//! - `Medium`; `y2025m03d25`; `Independently Solved`; `3ms`; `2.5mb`; `1 attempt`;
//! Topics: graph/bfs.
//!
//! I was busy on March 22nd, 2025 and copied the official editorial "cpp" solution at first.

pub fn count_complete_components(n: i32, edges: Vec<Vec<i32>>) -> i32 {
    use std::collections::VecDeque;
    let node_num = n as usize;
    let mut adj_vec_arr_box: Box<[Vec<usize>]> =
        vec![Vec::with_capacity(node_num); node_num].into_boxed_slice();

    for edge_vec in edges {
        let node_a_idx = edge_vec[0] as usize;
        let node_b_idx = edge_vec[1] as usize;

        adj_vec_arr_box[node_a_idx].push(node_b_idx);
        adj_vec_arr_box[node_b_idx].push(node_a_idx);
    }

    let adj_vec_arr_box = adj_vec_arr_box;

    let mut visited_vec: Vec<bool> = vec![false; node_num];

    let mut node_idx_queue: VecDeque<usize> = VecDeque::with_capacity(node_num);

    let mut ans_num: usize = 0;

    for node_i in 0..node_num {
        if visited_vec[node_i] == true {
            continue;
        }

        visited_vec[node_i] = true;

        node_idx_queue.push_back(node_i);

        let begin_next_node_idx_arr_ref = adj_vec_arr_box[node_i].as_slice();

        let group_node_num = begin_next_node_idx_arr_ref.len();

        for begin_next_node_idx in begin_next_node_idx_arr_ref.iter().cloned() {
            visited_vec[begin_next_node_idx] = true;
            node_idx_queue.push_back(begin_next_node_idx);
        }

        let mut complete_flag = true;

        while let Some(curr_node_idx) = node_idx_queue.pop_front() {
            let curr_next_node_idx_arr_ref = adj_vec_arr_box[curr_node_idx].as_slice();

            if curr_next_node_idx_arr_ref.len() != group_node_num {
                complete_flag = false
            }

            for curr_next_node_idx in curr_next_node_idx_arr_ref.iter().cloned() {
                if visited_vec[curr_next_node_idx] == true {
                    continue;
                }

                complete_flag = false;

                visited_vec[curr_next_node_idx] = true;
                node_idx_queue.push_back(curr_next_node_idx);
            }
        }

        if complete_flag {
            ans_num += 1;
        }
    }

    ans_num as i32
}
