//! # Leetcode 2658. Maximum Number of Fish in a Grid
//! https://leetcode.com/problems/maximum-number-of-fish-in-a-grid/
//! - `Medium`; `y2025m01d28`; `Independently Solved`; `2ms`; `2.3mb`; `1 attempt`;

pub fn find_max_fish(mut grid: Vec<Vec<i32>>) -> i32 {
    use std::collections::VecDeque;
    let row_num = grid.len();
    let col_num = grid.first().expect("len > 0").len();
    let calc_index_fn = |row_i: usize, col_i: usize| -> usize { row_i * col_num + col_i };
    let calc_pos_fn = |idx: usize| -> (usize, usize) { (idx / col_num, idx % col_num) };

    let mut waitlist_queue: VecDeque<usize> = VecDeque::new();

    let bfs_helper = |grid_mut_ref: &mut [Vec<i32>], queue_mut_ref: &mut VecDeque<usize>| -> i32 {
        let mut ans_sum: i32 = 0;
        while queue_mut_ref.is_empty() == false {
            let curr_idx = queue_mut_ref.pop_front().expect("Checked len...");
            let (curr_row_i, curr_col_i) = calc_pos_fn(curr_idx);

            if curr_row_i > 0 {
                let next_row_i = curr_row_i - 1;
                let next_entry = grid_mut_ref
                    .get_mut(next_row_i)
                    .expect("checked len.")
                    .get_mut(curr_col_i)
                    .expect("checked len");
                if *next_entry > 0 {
                    ans_sum += *next_entry;
                    *next_entry = 0;
                    queue_mut_ref.push_back(calc_index_fn(next_row_i, curr_col_i));
                }
            }

            if curr_row_i + 1 < row_num {
                let next_row_i = curr_row_i + 1;
                let next_entry = grid_mut_ref
                    .get_mut(next_row_i)
                    .expect("checked len.")
                    .get_mut(curr_col_i)
                    .expect("checked len");
                if *next_entry > 0 {
                    ans_sum += *next_entry;
                    *next_entry = 0;
                    queue_mut_ref.push_back(calc_index_fn(next_row_i, curr_col_i));
                }
            }

            if curr_col_i > 0 {
                let next_col_i = curr_col_i - 1;
                let next_entry = grid_mut_ref
                    .get_mut(curr_row_i)
                    .expect("checked len.")
                    .get_mut(next_col_i)
                    .expect("checked len");
                if *next_entry > 0 {
                    ans_sum += *next_entry;
                    *next_entry = 0;
                    queue_mut_ref.push_back(calc_index_fn(curr_row_i, next_col_i));
                }
            }

            if curr_col_i + 1 < col_num {
                let next_col_i = curr_col_i + 1;
                let next_entry = grid_mut_ref
                    .get_mut(curr_row_i)
                    .expect("checked len.")
                    .get_mut(next_col_i)
                    .expect("checked len");
                if *next_entry > 0 {
                    ans_sum += *next_entry;
                    *next_entry = 0;
                    queue_mut_ref.push_back(calc_index_fn(curr_row_i, next_col_i));
                }
            }
        }
        ans_sum
    };

    let mut ans_max_sum = 0;

    for row_i in 0..row_num {
        for col_i in 0..col_num {
            if grid[row_i][col_i] == 0 {
                continue;
            }
            let first_val = grid[row_i][col_i];
            grid[row_i][col_i] = 0;
            waitlist_queue.push_back(calc_index_fn(row_i, col_i));

            ans_max_sum = ans_max_sum.max(bfs_helper(&mut grid, &mut waitlist_queue) + first_val);

            waitlist_queue.clear();
        }
    }
    ans_max_sum
}
