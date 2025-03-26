//! # Leetcode 1976. Number of Ways to Arrive at Destination
//! https://leetcode.com/problems/number-of-ways-to-arrive-at-destination/\
//! - `Medium`; `y2025m03d25`; `Learned from Solution`; `0ms`; `3.7ms`; `2 attempt`;
//! Topics: graph/dijakastra
//!
//! I was busy on March 23rd, 2025, and copied the official "cpp" editorial at first.
//!
//! I forgot that I could just "part with" the staled distance heap entry instead of designing an updatable heap.

pub fn count_paths(n: i32, roads: Vec<Vec<i32>>) -> i32 {
    use std::collections::BinaryHeap;

    let node_num = n as usize;
    let mut adj_vec_arr_box: Box<[Vec<usize>]> =
        vec![Vec::with_capacity(node_num); node_num].into_boxed_slice();

    for (edge_idx, edge_vec_ref) in roads.iter().enumerate() {
        let node_a_idx = edge_vec_ref[0] as usize;
        let node_b_idx = edge_vec_ref[1] as usize;

        adj_vec_arr_box[node_a_idx].push(edge_idx * 2);
        adj_vec_arr_box[node_b_idx].push(edge_idx * 2 + 1);
    }

    let adj_vec_arr_box = adj_vec_arr_box;

    let edge_node_and_cost_closure = |edge_info_idx: usize| -> (usize, i32) {
        let first_node_flag = (edge_info_idx & 1) == 0;
        let edge_idx = edge_info_idx >> 1;

        let edge_ref = roads[edge_idx].as_slice();

        if first_node_flag {
            (edge_ref[1] as usize, edge_ref[2])
        } else {
            (edge_ref[0] as usize, edge_ref[2])
        }
    };

    let mut node_info_arr_box: Box<[(i64, usize)]> =
        vec![(i64::MAX, 0); node_num].into_boxed_slice(); // Even the distance needs `i64` haha.

    node_info_arr_box[0].0 = 0;
    node_info_arr_box[0].1 = 1;

    let mut node_idx_and_cost_pq: BinaryHeap<(i64, usize)> = BinaryHeap::with_capacity(node_num);

    node_idx_and_cost_pq.push((0, 0));
    const MODULOS: usize = 1000000007;

    while let Some((node_cost, node_idx)) = node_idx_and_cost_pq.pop() {
        let node_cost = -node_cost; // Max heap haha.
        if node_info_arr_box[node_idx].0 > node_cost {
            continue;
        }
        let node_way_num = node_info_arr_box[node_idx].1;
        let next_arr_ref = adj_vec_arr_box[node_idx].as_slice();

        for (next_node_idx, edge_cost) in
            next_arr_ref.iter().cloned().map(edge_node_and_cost_closure)
        {
            let next_cost = node_cost + edge_cost as i64;
            let next_node_info_entry_mut_ref = &mut node_info_arr_box[next_node_idx];
            if next_cost > next_node_info_entry_mut_ref.0 {
                continue;
            } else if next_cost == next_node_info_entry_mut_ref.0 {
                next_node_info_entry_mut_ref.1 =
                    (next_node_info_entry_mut_ref.1 + node_way_num) % MODULOS;
                continue;
            }

            next_node_info_entry_mut_ref.0 = next_cost;
            next_node_info_entry_mut_ref.1 = node_way_num;

            node_idx_and_cost_pq.push((-next_cost, next_node_idx));
        }
    }

    node_info_arr_box[node_num - 1].1 as i32
}
