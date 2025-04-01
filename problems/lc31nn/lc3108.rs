//! # Leetcode 3108. Minimum Cost Walk in Weighted Graph
//! https://leetcode.com/problems/minimum-cost-walk-in-weighted-graph/
//! - `Hard`; `y2025m03d25`; `Learned from Solution`; `23ms`; `22.8mb`; `1 attempt`;
//! Topics: graph/bfs.
//!
//! I was busy on March 20th, 2025, and copied the official "cpp" editorial at first.
//!
//! I didn't realize the minimum walk between nodes is just the sum of the "and" operation of all edges within a component.

pub fn minimum_cost(n: i32, edges: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
    use std::collections::VecDeque;

    let node_num = n as usize;
    let mut adj_vec_arr_box: Box<[Vec<usize>]> =
        vec![Vec::with_capacity(node_num); node_num].into_boxed_slice();

    for (edge_idx, edge_vec_ref) in edges.iter().enumerate() {
        let node_a_idx = edge_vec_ref[0] as usize;
        let node_b_idx = edge_vec_ref[1] as usize;

        adj_vec_arr_box[node_a_idx].push(edge_idx * 2);
        adj_vec_arr_box[node_b_idx].push(edge_idx * 2 + 1);
    }

    let adj_vec_arr_box = adj_vec_arr_box;

    let mut component_cost_vec: Vec<i32> = Vec::with_capacity(node_num);

    let mut node_belong_component_idx_arr_box: Box<[usize]> =
        vec![node_num; node_num].into_boxed_slice();

    let mut node_idx_queue: VecDeque<usize> = VecDeque::with_capacity(node_num);

    let edge_node_and_cost_closure = |edge_info_idx: usize| -> (usize, i32) {
        let first_node_flag = (edge_info_idx & 1) == 0;
        let edge_idx = edge_info_idx >> 1;

        let edge_ref = edges[edge_idx].as_slice();

        if first_node_flag {
            (edge_ref[1] as usize, edge_ref[2])
        } else {
            (edge_ref[0] as usize, edge_ref[2])
        }
    };

    for node_i in 0..node_num {
        if node_belong_component_idx_arr_box[node_i] != node_num {
            continue;
        }

        let component_idx = component_cost_vec.len();
        let mut component_cost: i32 = -1;

        node_idx_queue.push_back(node_i);

        node_belong_component_idx_arr_box[node_i] = component_idx;

        while let Some(curr_node_idx) = node_idx_queue.pop_front() {
            let curr_next_node_idx_arr_ref = adj_vec_arr_box[curr_node_idx].as_slice();

            for (curr_next_node_idx, edge_cost) in curr_next_node_idx_arr_ref
                .iter()
                .cloned()
                .map(edge_node_and_cost_closure)
            {
                // println!("{}->{}, cost:{}", curr_node_idx, curr_next_node_idx, edge_cost);

                component_cost &= edge_cost;

                if node_belong_component_idx_arr_box[curr_next_node_idx] != node_num {
                    continue;
                }

                node_belong_component_idx_arr_box[curr_next_node_idx] = component_idx;
                node_idx_queue.push_back(curr_next_node_idx);
            }
        }

        component_cost_vec.push(component_cost);
    }

    // println!("{:?}", component_cost_vec);

    let query_num = queries.len();

    let mut ans_vec: Vec<i32> = Vec::with_capacity(query_num);

    for query_vec in queries {
        let node_a_idx = query_vec[0] as usize;
        let node_b_idx = query_vec[1] as usize;

        let node_a_component_idx = node_belong_component_idx_arr_box[node_a_idx];
        let node_b_component_idx = node_belong_component_idx_arr_box[node_b_idx];

        if node_a_component_idx != node_b_component_idx {
            ans_vec.push(-1);
        } else {
            ans_vec.push(component_cost_vec[node_a_component_idx]);
        }
    }

    ans_vec
}
