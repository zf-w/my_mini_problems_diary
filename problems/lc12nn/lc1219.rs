//! ## Leetcode 1219. Path With Maximum Gold
//! https://leetcode.com/problems/path-with-maximum-gold
//! - `Medium`; `Independently Solved`; `2024-05-14`;
//!
//! We can use Depth-First-Search to solve this problem. One interesting part is that we can use the negative signs of elements in the grid array as a map for visited positions.

fn dfs(grid: &mut Vec<Vec<i32>>, row_i: usize, col_i: usize) -> i32 {
    let row_len = grid.len();
    let col_len = grid[0].len();
    let curr_v = grid[row_i][col_i];
    grid[row_i][col_i] = -curr_v;

    let mut next_max_v = 0;

    if row_i > 0 && grid[row_i - 1][col_i] > 0 {
        next_max_v = next_max_v.max(dfs(grid, row_i - 1, col_i));
    }

    if row_i < row_len - 1 && grid[row_i + 1][col_i] > 0 {
        next_max_v = next_max_v.max(dfs(grid, row_i + 1, col_i));
    }

    if col_i > 0 && grid[row_i][col_i - 1] > 0 {
        next_max_v = next_max_v.max(dfs(grid, row_i, col_i - 1));
    }

    if col_i < col_len - 1 && grid[row_i][col_i + 1] > 0 {
        next_max_v = next_max_v.max(dfs(grid, row_i, col_i + 1));
    }

    grid[row_i][col_i] = curr_v;
    curr_v + next_max_v
}

pub fn get_maximum_gold(mut grid: Vec<Vec<i32>>) -> i32 {
    let row_len = grid.len();
    let col_len = grid[0].len();
    let mut ans_max = 0;
    for row_i in 0..row_len {
        for col_i in 0..col_len {
            if grid[row_i][col_i] > 0 {
                ans_max = ans_max.max(dfs(&mut grid, row_i, col_i));
            }
        }
    }

    ans_max
}
