//! # Leetcode 2017. Grid Game
//! https://leetcode.com/problems/grid-game/
//! - `Medium`; `y2025m01d21`; `Hinted`; `0ms`; `3.4mb`; `2 attempts`;

pub fn grid_game(grid: Vec<Vec<i32>>) -> i64 {
    fn i32_ref_to_i64(i32_ref: &i32) -> i64 {
        (*i32_ref) as i64
    }
    let row_0_sum: i64 = grid[0].iter().map(i32_ref_to_i64).sum();
    // let row_1_sum: i64 = grid[1].iter().map(i32_ref_to_i64).sum();

    let col_num = grid[0].len();
    let mut row_0_curr_sum: i64 = 0;
    let mut row_1_curr_sum: i64 = 0;
    let mut ans_sum: i64 = 0;
    for col_i in 0..col_num {
        row_0_curr_sum += grid[0][col_i] as i64;
        let row_0_remain = row_0_sum - row_0_curr_sum;
        ans_sum = ans_sum.max(row_0_remain.max(row_1_curr_sum));
        row_1_curr_sum += grid[1][col_i] as i64;
    }
    ans_sum
}

pub fn grid_game_1(grid: Vec<Vec<i32>>) -> i64 {
    let row_num = grid.len();
    let col_num = grid[0].len();
    let mut grid_1: Vec<Vec<i64>> = grid
        .iter()
        .map(|row_ref| -> Vec<i64> {
            row_ref
                .iter()
                .map(|cell_ref| -> i64 { (*cell_ref) as i64 })
                .collect()
        })
        .collect();

    fn make_dp_vec_helper(grid_mut_ref: &mut [Vec<i64>]) {
        let row_num = grid_mut_ref.len();
        let col_num = grid_mut_ref[0].len();
        for row_i in 1..row_num {
            grid_mut_ref[row_i][0] += grid_mut_ref[row_i - 1][0];
        }

        for col_i in 1..col_num {
            grid_mut_ref[0][col_i] += grid_mut_ref[0][col_i - 1];
        }

        for row_i in 1..row_num {
            for col_i in 1..col_num {
                grid_mut_ref[row_i][col_i] +=
                    grid_mut_ref[row_i - 1][col_i].max(grid_mut_ref[row_i][col_i - 1]);
            }
        }
    }

    make_dp_vec_helper(&mut grid_1);

    println!("{}", grid_1[row_num - 1][col_num - 1]);

    let mut curr_row_i: usize = row_num - 1;
    let mut curr_col_i: usize = col_num - 1;

    grid_1[curr_row_i][curr_col_i] = 0;
    while curr_row_i > 0 || curr_col_i > 0 {
        if curr_row_i > 0 && curr_col_i > 0 {
            if grid_1[curr_row_i - 1][curr_col_i] > grid_1[curr_row_i][curr_col_i - 1] {
                curr_row_i -= 1;
            } else {
                curr_col_i -= 1;
            }
        } else if curr_row_i > 0 {
            curr_row_i -= 1;
        } else {
            curr_col_i -= 1;
        }
        grid_1[curr_row_i][curr_col_i] = 0;
    }

    for row_i in 0..row_num {
        for col_i in 0..col_num {
            if grid_1[row_i][col_i] > 0 {
                grid_1[row_i][col_i] = grid[row_i][col_i] as i64;
            }
            print!("{} ", grid_1[row_i][col_i]);
        }
        println!("");
    }

    make_dp_vec_helper(&mut grid_1);

    grid_1[row_num - 1][col_num - 1]
}
