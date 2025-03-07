//! Leetcode 1289. Minimum Falling Path Sum II
//! https://leetcode.com/problems/minimum-falling-path-sum-ii
//! - `Hard`; `Learned from Solution`; `2024-04-25`;
//!
//! We need to find the two smallest elements of each row and use dynamic programming to solve the problem. Another interesting technique I applied beyond "dp optimization" was to use an index function to achieve using one "1D" array to store the information for the dynamic programming. For memory safety considerations in Rust, I used an index function to switch between the previous column DP data and the current column DP data without additional copying.

pub fn min_falling_path_sum(grid: Vec<Vec<i32>>) -> i32 {
    // Busy, will be back
    let row_len = grid.len();
    let col_len = grid[0].len();
    let mut dp: Vec<i32> = vec![0; col_len * 2];
    let mut prev_i = 0;
    let mut curr_i = 1;
    let index = |part_i: usize, col_i: usize| -> usize { part_i * col_len + col_i };
    for row_i in 0..row_len {
        let mut row_min_0: Option<i32> = None;
        let mut row_min_1: Option<i32> = None;
        for col_i in 0..col_len {
            let prev_v = dp[index(prev_i, col_i)];
            match (row_min_0, row_min_1) {
                (None, None) => row_min_0 = Some(prev_v),
                (None, Some(_)) => (),
                (Some(row_min_0_v), None) => {
                    if prev_v < row_min_0_v {
                        row_min_1 = row_min_0;
                        row_min_0 = Some(prev_v);
                    } else {
                        row_min_1 = Some(prev_v);
                    }
                }
                (Some(row_min_0_v), Some(row_min_1_v)) => {
                    if prev_v < row_min_0_v {
                        row_min_1 = row_min_0;
                        row_min_0 = Some(prev_v);
                    } else if prev_v < row_min_1_v {
                        row_min_1 = Some(prev_v);
                    }
                }
            }
        }
        for col_i in 0..col_len {
            let curr_v = dp[index(prev_i, col_i)];
            let curr_min = if curr_v == row_min_0.expect("At least one") {
                row_min_1
            } else {
                row_min_0
            }
            .unwrap_or(i32::MAX);
            dp[index(curr_i, col_i)] = grid[row_i][col_i] + curr_min;
        }
        curr_i = prev_i;
        prev_i = (prev_i + 1) % 2;
    }
    let mut ans = dp[index(prev_i, 0)];
    for col_i in 1..col_len {
        ans = ans.min(dp[index(prev_i, col_i)]);
    }
    ans
}
