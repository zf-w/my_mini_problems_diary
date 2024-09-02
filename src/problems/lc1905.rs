//! ## Leetcode 2699. Modify Graph Edge Weights
//! https://leetcode.com/problems/modify-graph-edge-weights/
//! - `Hard`; `Learned from Solution`; `2024-08-28`;
//!
//! Learned from Solution: https://leetcode.com/problems/modify-graph-edge-weights/solutions/5708699/dijkstra-s-with-tc-o-e-v-log-v-beats-100-in-all-languages
//! Busy, currently partially copied, will be back.

use std::collections::VecDeque;

pub fn count_sub_islands(grid1: Vec<Vec<i32>>, grid2: Vec<Vec<i32>>) -> i32 {
    let row_len = grid1.len();
    let col_len = grid1[0].len();
    let mut visited_vec: Vec<bool> = vec![false; row_len * col_len];
    let pos_to_i_fn = |row_i: usize, col_i: usize| -> usize { row_i * col_len + col_i };
    let i_to_pos_fn = |i: usize| -> (usize, usize) { (i / col_len, i % col_len) };
    let mut wait_dq: VecDeque<usize> = VecDeque::new();
    let mut ans_count = 0;

    fn is_grid_cell_val_island(grid: &Vec<Vec<i32>>, row_i: usize, col_i: usize) -> bool {
        *grid.get(row_i).unwrap().get(col_i).unwrap() == 1
    }

    for row_i in 0..row_len {
        for col_i in 0..col_len {
            if is_grid_cell_val_island(&grid2, row_i, col_i) == false
                || visited_vec[pos_to_i_fn(row_i, col_i)] == true
            {
                continue;
            }
            let curr_i = pos_to_i_fn(row_i, col_i);
            wait_dq.push_back(curr_i);
            visited_vec[curr_i] = true;
            let mut sub_island_flag = is_grid_cell_val_island(&grid1, row_i, col_i);
            while wait_dq.is_empty() == false {
                let (curr_row_i, curr_col_i) =
                    i_to_pos_fn(wait_dq.pop_front().expect("Getting front: checked length"));

                if curr_row_i > 0 {
                    let next_row_i = curr_row_i - 1;

                    if is_grid_cell_val_island(&grid2, next_row_i, curr_col_i) {
                        visited_vec[pos_to_i_fn(next_row_i, curr_col_i)] = true;
                        if is_grid_cell_val_island(&grid1, next_row_i, curr_col_i) {
                            sub_island_flag = false;
                        }
                        wait_dq.push_back(pos_to_i_fn(next_row_i, curr_col_i));
                    }
                }

                if curr_col_i > 0 {
                    let next_col_i = curr_col_i - 1;

                    if is_grid_cell_val_island(&grid2, curr_row_i, next_col_i) {
                        visited_vec[pos_to_i_fn(curr_row_i, next_col_i)] = true;
                        if is_grid_cell_val_island(&grid1, curr_row_i, next_col_i) {
                            sub_island_flag = false;
                        }
                        wait_dq.push_back(pos_to_i_fn(curr_row_i, next_col_i));
                    }
                }

                if curr_row_i + 1 < row_len {
                    let next_row_i = curr_row_i + 1;

                    if is_grid_cell_val_island(&grid2, next_row_i, curr_col_i) {
                        visited_vec[pos_to_i_fn(next_row_i, curr_col_i)] = true;
                        if is_grid_cell_val_island(&grid1, next_row_i, curr_col_i) {
                            sub_island_flag = false;
                        }
                        wait_dq.push_back(pos_to_i_fn(next_row_i, curr_col_i));
                    }
                }

                if curr_col_i + 1 < col_len {
                    let next_col_i = curr_col_i + 1;

                    if is_grid_cell_val_island(&grid2, curr_row_i, next_col_i) {
                        visited_vec[pos_to_i_fn(curr_row_i, next_col_i)] = true;
                        if is_grid_cell_val_island(&grid1, curr_row_i, next_col_i) {
                            sub_island_flag = false;
                        }
                        wait_dq.push_back(pos_to_i_fn(curr_row_i, next_col_i));
                    }
                }
            }
            if sub_island_flag == true {
                ans_count += 1;
            }
        }
    }
    ans_count
}
