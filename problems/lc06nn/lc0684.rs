//! # Leetcode 684. Redundant Connection
//! https://leetcode.com/problems/redundant-connection/
//! - `Medium`; `y2025m01d29`; `Independently Solved`; `0ms`; `2.4mb`; `1 attempt`;

struct UnionFind {
    v: Vec<usize>,
}

impl UnionFind {
    pub fn new_with_size(size: usize) -> Self {
        let mut v = Vec::with_capacity(size);
        for i in 0..size {
            v.push(i);
        }
        Self { v }
    }

    pub fn find(&mut self, i: usize) -> usize {
        let prev = self.v[i];
        if prev == i {
            return i;
        }
        let prev = self.find(prev);
        self.v[i] = prev.clone();
        prev
    }

    pub fn union(&mut self, i: usize, j: usize) -> bool {
        let prev_i = self.find(i);
        let prev_j = self.find(j);

        if prev_i == prev_j {
            return false;
        }

        self.v[prev_j] = prev_i.clone();
        true
    }
}

pub fn find_redundant_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
    let len = edges
        .iter()
        .map(|edge_ref| -> usize { edge_ref[0].max(edge_ref[1]) as usize })
        .max()
        .expect("Should have an edge...");
    let mut union_find = UnionFind::new_with_size(len);
    for edge in edges {
        let node_a_i = edge[0] as usize - 1;
        let node_b_i = edge[1] as usize - 1;
        if union_find.union(node_a_i, node_b_i) == false {
            return edge;
        }
    }
    unreachable!()
}
