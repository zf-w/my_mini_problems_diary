//! ## Leetcode 1791. Find Center of Star Graph
//! https://leetcode.com/problems/find-center-of-star-graph/
//! - `Easy`; `Independently Solved`; `2024-06-24`;
//!
//! My first thought was to check which node would occur more than once. Later, after I have checked out other sample solutions from the complexity histogram, I realized that the node is inside all the edges, and we can find it out by comparing it with the nodes from another edge.

use std::collections::HashSet;

pub fn find_center(edges: Vec<Vec<i32>>) -> i32 {
    let mut counts: HashSet<i32> = HashSet::new();
    for edge in edges.iter() {
        let a = edge[0];
        let b = edge[1];
        if counts.contains(&a) {
            return a;
        } else {
            counts.insert(a);
        }
        if counts.contains(&b) {
            return b;
        } else {
            counts.insert(b);
        }
    }
    unreachable!()
}

pub fn find_center_1(edges: Vec<Vec<i32>>) -> i32 {
    let edge_0 = &edges[0];
    let edge_1 = &edges[1];
    let node_a = edge_0[0];
    let node_b = edge_0[1];
    if node_a == edge_1[0] || node_a == edge_1[1] {
        node_a
    } else {
        node_b
    }
}
