//! # Leetcode 2033. Minimum Operations to Make a Uni-Value Grid
//! https://leetcode.com/problems/minimum-operations-to-make-a-uni-value-grid/
//! - `Medium`; `y2025m03d25`; `Learned from Solution`; `2ms`; `8.5mb`; `5 attempts`;
//! Topics: uncategorized.
//! Learned from editorial: https://leetcode.com/problems/minimum-operations-to-make-a-uni-value-grid/editorial.
//! I thought I should use "mean" hahahaha.

pub fn min_operations(grid: Vec<Vec<i32>>, x: i32) -> i32 {
    let cell_num = grid.len() * grid[0].len();
    let prev_modulos = grid[0][0] % x;

    let mut num_vec: Vec<i32> = Vec::with_capacity(cell_num);

    for row in grid.iter() {
        for cell in row.iter().cloned() {
            let modulos = cell % x;
            if modulos != prev_modulos {
                return -1;
            }
            num_vec.push(cell);
        }
    }
    let median_idx = cell_num / 2;
    num_vec.select_nth_unstable(median_idx);

    let median = num_vec[median_idx];
    let mut ans = 0;

    for num in num_vec {
        ans += (num - median).abs() / x;
    }

    ans
}
