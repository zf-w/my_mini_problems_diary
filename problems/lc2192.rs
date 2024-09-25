//! ## Leetcode 2192. All Ancestors of a Node in a Directed Acyclic Graph
//! https://leetcode.com/problems/all-ancestors-of-a-node-in-a-directed-acyclic-graph/
//! - `Medium`; `Independently Solved`; `2024-06-28`;
//!
//! We can know all the ancestors of a node if all immediate "parents"' ancestors are known. Therefore, we can start from nodes having zero in-ward edges and propagate along the results.

use std::collections::{BTreeSet, VecDeque};

pub fn get_ancestors(n: i32, edges: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let n = n as usize;
    let mut graph_adj: Vec<Vec<usize>> = vec![vec![]; n];
    let mut in_edge_counts_vec: Vec<usize> = vec![0; n];
    for edge in edges.iter() {
        let from_node_i = edge[0] as usize;
        let to_node_i = edge[1] as usize;
        graph_adj[from_node_i].push(to_node_i);
        in_edge_counts_vec[to_node_i] += 1;
    }
    let mut wait_queue: VecDeque<usize> = VecDeque::with_capacity(n);
    for (curr_i, in_edge_count_ref) in in_edge_counts_vec.iter().enumerate() {
        if *in_edge_count_ref == 0 {
            wait_queue.push_back(curr_i);
        }
    }

    let mut ancestors_vec: Vec<BTreeSet<usize>> = vec![BTreeSet::new(); n];
    let mut temp_vec: Vec<usize> = Vec::with_capacity(n);
    while let Some(curr_node_i) = wait_queue.pop_front() {
        let next_nodes_ref = &graph_adj[curr_node_i];
        for next_node_i in next_nodes_ref.iter().cloned() {
            for curr_node_ancestor in &ancestors_vec[curr_node_i] {
                temp_vec.push(*curr_node_ancestor);
            }
            temp_vec.push(curr_node_i);
            let next_ancestors_mut_ref = &mut ancestors_vec[next_node_i];
            for value in temp_vec.iter().cloned() {
                next_ancestors_mut_ref.insert(value);
            }
            temp_vec.clear();
        }
        for next_node_i in next_nodes_ref.iter().cloned() {
            in_edge_counts_vec[next_node_i] -= 1;
            if in_edge_counts_vec[next_node_i] == 0 {
                wait_queue.push_back(next_node_i);
            }
        }
    }

    let mut ans_vec: Vec<Vec<i32>> = Vec::with_capacity(n);

    for node_ancestors_ref in ancestors_vec.iter() {
        let curr_ancestors = Vec::from_iter(
            node_ancestors_ref
                .iter()
                .map(|v| -> i32 { v.clone() as i32 }),
        );

        ans_vec.push(curr_ancestors);
    }

    ans_vec
}
