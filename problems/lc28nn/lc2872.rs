//! # Leetcode 2872. Maximum Number of K-Divisible Components
//! https://leetcode.com/problems/maximum-number-of-k-divisible-components/
//! - `Hard`; `y2025m11d27`; `Learned from Solution`; `24ms`; `8.4mb`; `1 attempt`;
//! Topics: tree.
//! https://leetcode.com/problems/maximum-number-of-k-divisible-components/editorial

pub fn max_k_divisible_components(n: i32, edges: Vec<Vec<i32>>, values: Vec<i32>, k: i32) -> i32 {
    let node_num = n as usize;

    fn edge_vec_to_edge_pair(edge_vec: Vec<i32>) -> (usize, usize) {
        (edge_vec[0] as usize, edge_vec[1] as usize)
    }

    let mut node_adj_vec_vec: Vec<Vec<usize>> = vec![Vec::with_capacity(node_num); node_num];

    for (node_i_0, node_i_1) in edges.into_iter().map(edge_vec_to_edge_pair) {
        node_adj_vec_vec[node_i_0].push(node_i_1);
        node_adj_vec_vec[node_i_1].push(node_i_0);
    }

    let node_adj_vec_vec = node_adj_vec_vec;

    fn helper_fn(
        parent_node_i: usize,
        curr_node_i: usize,
        static_info_tuple: (i32, &[i32], &[Vec<usize>]),
    ) -> (i32, usize) {
        let (k, values, node_adj_vec_arr_ref) = static_info_tuple;
        let mut component_num = 0;
        let mut value_sum = 0;

        for subtree_root_node_i in node_adj_vec_arr_ref[curr_node_i].iter().cloned() {
            if subtree_root_node_i == parent_node_i {
                continue;
            }

            let (subtree_value_sum, subtree_component_num) =
                helper_fn(curr_node_i, subtree_root_node_i, static_info_tuple);

            value_sum = (value_sum + subtree_value_sum) % k;
            component_num += subtree_component_num;
        }

        let curr_node_value = values[curr_node_i];

        value_sum = (value_sum + curr_node_value) % k;

        if value_sum == 0 {
            component_num += 1;
        }

        (value_sum, component_num)
    }

    helper_fn(node_num + 1, 0, (k, &values, node_adj_vec_vec.as_slice())).1 as i32
}