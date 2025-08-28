//! # Leetcode 3459. Length of Longest V-Shaped Diagonal Segment
//! https://leetcode.com/problems/length-of-longest-v-shaped-diagonal-segment/
//! - `hard`; `y2025m08d27`; `Learned from Solution`; `63ms`; `12mb`; `1 attempt`;
//! Topics: dynamic_programming.

pub fn len_of_v_diagonal(grid: Vec<Vec<i32>>) -> i32 {
    let row_num = grid.len();
    let col_num = grid[0].len();

    let dir_update_fn_arr: [fn(usize, usize) -> (usize, usize); 4] = [
        |row_num, col_num| -> (usize, usize) { (row_num + 1, col_num + 1) },
        |row_num, col_num| -> (usize, usize) { (row_num + 1, col_num - 1) },
        |row_num, col_num| -> (usize, usize) { (row_num - 1, col_num - 1) },
        |row_num, col_num| -> (usize, usize) { (row_num - 1, col_num + 1) },
    ];

    let mut dp: Vec<i32> = vec![-1; row_num * col_num * 4 * 2];

    let row_mul = col_num * 4 * 2;
    let col_mul = 4 * 2;
    let index_fn = |row_i: usize, col_i: usize, dir_idx: usize, turn_idx: usize| -> usize {
        row_i * row_mul + col_i * col_mul + dir_idx * 2 + turn_idx
    };

    fn dfs_fn(
        row_i: usize,
        col_i: usize,
        dir_idx: usize,
        turn_idx: usize,
        target: i32,
        grid: &Vec<Vec<i32>>,
        dir_update_fn_arr: &[fn(usize, usize) -> (usize, usize); 4],
        index_fn: &impl Fn(usize, usize, usize, usize) -> usize,
        dp: &mut Vec<i32>,
    ) -> i32 {
        let row_num = grid.len();
        let col_num = grid[0].len();

        let (next_row_i, next_col_i) = dir_update_fn_arr[dir_idx](row_i, col_i);

        if next_row_i >= row_num || next_col_i >= col_num || grid[next_row_i][next_col_i] != target
        {
            return 0;
        }

        let entry_idx = index_fn(next_row_i, next_col_i, dir_idx, turn_idx);

        if dp[entry_idx] != -1 {
            return dp[entry_idx];
        }

        let mut max_step = dfs_fn(
            next_row_i,
            next_col_i,
            dir_idx,
            turn_idx,
            2 - target,
            grid,
            dir_update_fn_arr,
            index_fn,
            dp,
        );

        if turn_idx == 1 {
            max_step = max_step.max(dfs_fn(
                next_row_i,
                next_col_i,
                (dir_idx + 1) % 4,
                0,
                2 - target,
                grid,
                dir_update_fn_arr,
                index_fn,
                dp,
            ));
        }

        dp[entry_idx] = max_step + 1;

        max_step + 1
    };

    let mut ans = 0;

    for row_i in 0..row_num {
        for col_i in 0..col_num {
            if grid[row_i][col_i] != 1 {
                continue;
            }

            for dir_idx in 0..4 {
                ans = ans.max(
                    dfs_fn(
                        row_i,
                        col_i,
                        dir_idx,
                        1,
                        2,
                        &grid,
                        &dir_update_fn_arr,
                        &index_fn,
                        &mut dp,
                    ) + 1,
                );
            }
        }
    }

    ans
}
