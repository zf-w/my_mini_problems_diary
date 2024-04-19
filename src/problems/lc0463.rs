//! ## Leetcode 463. Island Perimeter
//! https://leetcode.com/problems/island-perimeter/
//! - `Easy`; `Independently Solved`; `2024-04-19`;
//!
//! I'm just scanning through the grids. Indeed, how to iterate through a grid with the information of neighbors in a natural and safe way is an interesting problem.

pub fn island_perimeter(grid: Vec<Vec<i32>>) -> i32 {
    let mut perimeter = 0;
    let row_last_i = grid.len() - 1;

    for (row_i, row) in grid.iter().enumerate() {
        let col_last_i = row.len() - 1;
        for (col_i, cell) in row.iter().enumerate() {
            if cell == &0 {
                continue;
            }
            if col_i == 0 || row[col_i - 1] == 1 {
                perimeter += 1;
            }
            if col_i < col_last_i || row[col_i + 1] == 1 {
                perimeter += 1;
            }
            if row_i == 0 || grid[row_i - 1][col_i] == 1 {
                perimeter += 1;
            }
            if row_i < row_last_i || grid[row_i - 1][col_i] == 1 {
                perimeter += 1;
            }
        }
    }
    perimeter
}
