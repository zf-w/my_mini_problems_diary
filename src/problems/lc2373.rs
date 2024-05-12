//! ## Leetcode 2373. Largest Local Values in a Matrix
//! https://leetcode.com/problems/largest-local-values-in-a-matrix
//! - `Easy`; `Independently Solved`; `2024-05-11`;
//!
//! We can loop through the matrix and find the max for each 3 * 3 block. I guess making a priority queue would be less beneficial, considering the block is only 3 * 3, meaning we need to pop 3 elements and add 3 elements for each shifting of block.

pub fn largest_local(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let row_len = grid.len();
    let col_len = grid[0].len(); // Should have
    let mut ans: Vec<Vec<i32>> = Vec::with_capacity(row_len);
    for row_i in 1..(row_len - 1) {
        let mut ans_row = Vec::with_capacity(col_len);
        for col_i in 1..(col_len - 1) {
            let mut max: i32 = 0;
            max = max.max(grid[row_i - 1][col_i - 1]);
            max = max.max(grid[row_i - 1][col_i]);
            max = max.max(grid[row_i - 1][col_i + 1]);
            max = max.max(grid[row_i][col_i - 1]);
            max = max.max(grid[row_i][col_i]);
            max = max.max(grid[row_i][col_i + 1]);
            max = max.max(grid[row_i + 1][col_i - 1]);
            max = max.max(grid[row_i + 1][col_i]);
            max = max.max(grid[row_i + 1][col_i + 1]);
            ans_row.push(max);
        }
        ans.push(ans_row);
    }
    ans
}
