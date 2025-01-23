//! # Leetcode 1765. Map of Highest Peak
//! https://leetcode.com/problems/map-of-highest-peak/
//! - `Medium`; `y2025m01d22`; `Independently Solved`; `10ms`, `14mb`; `1 attempt`;

pub fn highest_peak(mut is_water: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let row_num = is_water.len();
    let col_num = is_water[0].len();
    let cord_to_idx_fn = |row_i: usize, col_i: usize| -> usize { row_i * col_num + col_i };
    let idx_to_cord_fn = |idx: usize| -> (usize, usize) { (idx / col_num, idx % col_num) };

    use std::collections::VecDeque;

    let mut cell_idx_queue: VecDeque<usize> = VecDeque::with_capacity(row_num * col_num);

    for (row_i, row_mut_ref) in is_water.iter_mut().enumerate() {
        for (col_i, cell_mut_ref) in row_mut_ref.iter_mut().enumerate() {
            if *cell_mut_ref == 0 {
                *cell_mut_ref = -1;
            } else {
                *cell_mut_ref = 0;
                cell_idx_queue.push_back(cord_to_idx_fn(row_i, col_i));
            }
        }
    }

    let mut level = 1;

    while cell_idx_queue.is_empty() == false {
        let level_cell_num = cell_idx_queue.len();
        for _ in 0..level_cell_num {
            let curr_idx = cell_idx_queue
                .pop_front()
                .expect("popping queue front, checked length.");
            let (curr_row_i, curr_col_i) = idx_to_cord_fn(curr_idx);

            let mut next_row_i;
            let mut next_col_i;

            if curr_row_i > 0 {
                next_row_i = curr_row_i - 1;
                if is_water[next_row_i][curr_col_i] == -1 {
                    cell_idx_queue.push_back(cord_to_idx_fn(next_row_i, curr_col_i));
                    is_water[next_row_i][curr_col_i] = level;
                }
            }

            if curr_col_i > 0 {
                next_col_i = curr_col_i - 1;
                if is_water[curr_row_i][next_col_i] == -1 {
                    cell_idx_queue.push_back(cord_to_idx_fn(curr_row_i, next_col_i));
                    is_water[curr_row_i][next_col_i] = level;
                }
            }

            if curr_row_i < row_num - 1 {
                next_row_i = curr_row_i + 1;
                if is_water[next_row_i][curr_col_i] == -1 {
                    cell_idx_queue.push_back(cord_to_idx_fn(next_row_i, curr_col_i));
                    is_water[next_row_i][curr_col_i] = level;
                }
            }

            if curr_col_i < col_num - 1 {
                next_col_i = curr_col_i + 1;
                if is_water[curr_row_i][next_col_i] == -1 {
                    cell_idx_queue.push_back(cord_to_idx_fn(curr_row_i, next_col_i));
                    is_water[curr_row_i][next_col_i] = level;
                }
            }
        }
        level += 1;
    }

    is_water
}
