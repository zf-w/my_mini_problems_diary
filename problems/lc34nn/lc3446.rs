//! # Leetcode 3446. Sort Matrix by Diagonals
//! https://leetcode.com/problems/sort-matrix-by-diagonals/
//! - `Medium`; `y2025m08d27`; `Independently Solved`; `6ms`; `2.29mb`; `1 attempt`;
//! Topics: uncategorized.

pub fn sort_matrix(mut grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let row_num = grid.len();

    let mut sort_vec = Vec::with_capacity(row_num);

    for row_i in 0..row_num {
        let diagnal_len = row_num - row_i;
        for diag_i in 0..diagnal_len {
            sort_vec.push(grid[row_i + diag_i][diag_i]);
        }

        sort_vec.sort_unstable();
        for (diag_i, v) in sort_vec.iter().cloned().rev().enumerate() {
            grid[row_i + diag_i][diag_i] = v;
        }
        sort_vec.clear();
    }

    for col_i in 1..row_num {
        let diagnal_len = row_num - col_i;
        for diag_i in 0..diagnal_len {
            sort_vec.push(grid[diag_i][col_i + diag_i]);
        }

        sort_vec.sort_unstable();
        for (diag_i, v) in sort_vec.iter().cloned().enumerate() {
            grid[diag_i][col_i + diag_i] = v;
        }
        sort_vec.clear();
    }
    grid
}
