//! # Leetcode 2467. Most Profitable Path in a Tree
//! https://leetcode.com/problems/most-profitable-path-in-a-tree/
//! - `Medium`; `y2025m02d28`; `Independently Solved`; `27ms`; `19.7mb`; `1 attempt`;
//!
//! Note: I was busy and copied the solution from "https://leetcode.com/problems/most-profitable-path-in-a-tree/solutions/6415190/most-profitable-path-in-a-tree".
//!
//! Node: I think, instead of using a structure to keep track of visited nodes, building the tree structure prior might be another choice.

pub fn most_profitable_path(edges: Vec<Vec<i32>>, bob: i32, amount: Vec<i32>) -> i32 {
    use std::collections::VecDeque;
    let bob_begin_node_i = bob as usize;
    let node_num = amount.len();
    let mut adj_arr_box: Box<[(usize, Vec<usize>, usize)]> =
        vec![(node_num, Vec::new(), node_num); node_num].into_boxed_slice();

    for edge in edges {
        let node_a_i = edge[0] as usize;
        let node_b_i = edge[1] as usize;

        adj_arr_box[node_a_i].1.push(node_b_i);
        adj_arr_box[node_b_i].1.push(node_a_i);
    }

    let mut bfs_q: VecDeque<(usize, usize)> = VecDeque::with_capacity(node_num);

    bfs_q.push_back((0, node_num));

    while bfs_q.is_empty() == false {
        let (curr_node_i, parent_node_i) = bfs_q.pop_front().expect("Checked len...");
        // println!("{} {}", curr_node_i, parent_node_i);
        let next_node_idx_iter = adj_arr_box[curr_node_i].1.iter().cloned();
        for next_node_idx in next_node_idx_iter {
            if next_node_idx == parent_node_i {
                continue;
            }

            adj_arr_box[next_node_idx].0 = curr_node_i;
            bfs_q.push_back((next_node_idx, curr_node_i));
        }
    }

    let mut bob_node_i = bob_begin_node_i;
    let mut bob_count = 0;
    while bob_node_i < node_num {
        let node_mut_ref = &mut adj_arr_box[bob_node_i];
        node_mut_ref.2 = bob_count;

        // println!("{} {}", bob_node_i, bob_count);

        bob_node_i = node_mut_ref.0;

        bob_count += 1;
    }

    let mut ans_profit = -2147483648;

    fn dfs_helper(
        curr_node_i: usize,
        curr_profit: i32,
        curr_time: usize,
        ans_profit_mut_ref: &mut i32,
        adj_arr_ref: &[(usize, Vec<usize>, usize)],
        amount_ref: &[i32],
    ) {
        let node_ref = &adj_arr_ref[curr_node_i];
        let next_idx_arr_ref = node_ref.1.as_slice();

        let node_profit = amount_ref[curr_node_i];
        let bob_time = node_ref.2;
        let next_profit = curr_profit
            + if curr_time < bob_time {
                node_profit
            } else if curr_time == bob_time {
                node_profit / 2
            } else {
                0
            };

        if next_idx_arr_ref.len() == 1 && node_ref.0 != amount_ref.len() {
            *ans_profit_mut_ref = (*ans_profit_mut_ref).max(next_profit);
            return;
        }

        let next_time = curr_time + 1;

        for next_node_idx in next_idx_arr_ref.iter().cloned() {
            if next_node_idx == node_ref.0 {
                continue;
            }

            dfs_helper(
                next_node_idx,
                next_profit,
                next_time,
                ans_profit_mut_ref,
                adj_arr_ref,
                amount_ref,
            );
        }
    }

    dfs_helper(0, 0, 0, &mut ans_profit, &adj_arr_box, &amount);

    ans_profit
}
