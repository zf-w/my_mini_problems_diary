//! ## Leetcode 2812. Find the Safest Path in a Grid
//! https://leetcode.com/problems/find-the-safest-path-in-a-grid
//! - `Medium`; `Independently Solved`; `2024-05-14`;
//!
//! We can solve this problem by first using BFS to calculate the safety score of each cell and then finding the maximum safety score of the grid. I independently figured out the first part and a slow version of the second part. I was thinking about a version of BFS with the safety score of the current path, trying to not revisit cells with higher safety scores. My approach first assumes the highest safety score is zero and then updates it when finding a higher safety score.
//!
//! After my solution in Rust-lang has passed the test cases in 800ms, I read the solutions and found there is another approach that decreases the safety score from the safety score of the starting position and breaks after reaching the goal. This approach incorporates a strange way of pushing new nodes into the search mechanism. The approach seems to be combining both DFS and BFS. Maybe it is trying to search nodes with higher safety scores first?

use std::collections::VecDeque;

pub fn maximum_safeness_factor(mut grid: Vec<Vec<i32>>) -> i32 {
    let row_len = grid.len();
    let col_len = grid[0].len();
    let last_row_i = row_len - 1;
    let last_col_i = col_len - 1;
    let mut q: VecDeque<usize> = VecDeque::with_capacity(row_len * col_len);
    let cord_to_key = |row_i: usize, col_i: usize| -> usize { row_i * col_len + col_i };
    let key_to_cord = |key: usize| -> (usize, usize) { (key / col_len, key % col_len) };
    for (row_i, row) in grid.iter_mut().enumerate() {
        for (col_i, cell) in row.iter_mut().enumerate() {
            if *cell == 1 {
                q.push_back(cord_to_key(row_i, col_i));
                *cell = 0;
            } else {
                *cell = -1;
            }
        }
    }
    let mut dis = 1;
    while !q.is_empty() {
        let curr_len = q.len();
        for _ in 0..curr_len {
            let curr_key = q.pop_front().expect("Checked length");
            let (row_i, col_i) = key_to_cord(curr_key);

            if row_i > 0 && grid[row_i - 1][col_i] < 0 {
                q.push_back(cord_to_key(row_i - 1, col_i));
                grid[row_i - 1][col_i] = dis;
            }

            if row_i < last_row_i && grid[row_i + 1][col_i] < 0 {
                q.push_back(cord_to_key(row_i + 1, col_i));
                grid[row_i + 1][col_i] = dis;
            }

            if col_i > 0 && grid[row_i][col_i - 1] < 0 {
                q.push_back(cord_to_key(row_i, col_i - 1));
                grid[row_i][col_i - 1] = dis;
            }

            if col_i < last_col_i && grid[row_i][col_i + 1] < 0 {
                q.push_back(cord_to_key(row_i, col_i + 1));
                grid[row_i][col_i + 1] = dis;
            }
        }
        dis += 1;
    }
    // for (row_i, row) in grid.iter().enumerate() {
    //     for (col_i, cell) in row.iter().enumerate() {
    //         print!("{} ",*cell);
    //     }
    //     println!("");
    // }
    // let mut visited: Vec<Vec<i32>> = vec![vec![0; col_len]; row_len];
    let start_val = grid[0][0];
    // visited[0][0] = start_val;
    let mut q: VecDeque<(usize, i32)> = VecDeque::with_capacity(row_len * col_len);
    q.push_back((0, start_val));

    let mut ans_safety = start_val;
    while let Some((curr_key, prev_val)) = q.pop_front() {
        let (row_i, col_i) = key_to_cord(curr_key);
        // let curr_val = grid[row_i][col_i].min(prev_val);
        ans_safety = ans_safety.min(prev_val);

        // println!("{} {} {}", row_i, col_i, curr_val);
        if row_i == last_row_i && col_i == last_col_i {
            break;
        }

        if row_i > 0 && grid[row_i - 1][col_i] > -1 {
            let safety = grid[row_i - 1][col_i];
            grid[row_i - 1][col_i] = -1;
            if safety < ans_safety {
                q.push_back((cord_to_key(row_i - 1, col_i), safety));
            } else {
                q.push_front((cord_to_key(row_i - 1, col_i), safety));
            }
        }

        if row_i < last_row_i && grid[row_i + 1][col_i] > -1 {
            let safety = grid[row_i + 1][col_i];
            grid[row_i + 1][col_i] = -1;
            if safety < ans_safety {
                q.push_back((cord_to_key(row_i + 1, col_i), safety));
            } else {
                q.push_front((cord_to_key(row_i + 1, col_i), safety));
            }
        }

        if col_i > 0 && grid[row_i][col_i - 1] > -1 {
            let safety = grid[row_i][col_i - 1];
            grid[row_i][col_i - 1] = -1;
            if safety < ans_safety {
                q.push_back((cord_to_key(row_i, col_i - 1), safety));
            } else {
                q.push_front((cord_to_key(row_i, col_i - 1), safety));
            }
        }

        if col_i < last_col_i && grid[row_i][col_i + 1] > -1 {
            let safety = grid[row_i][col_i + 1];
            grid[row_i][col_i + 1] = -1;
            if safety < ans_safety {
                q.push_back((cord_to_key(row_i, col_i + 1), safety));
            } else {
                q.push_front((cord_to_key(row_i, col_i + 1), safety));
            }
        }
    }

    ans_safety
}

pub fn my_maximum_safeness_factor(mut grid: Vec<Vec<i32>>) -> i32 {
    let row_len = grid.len();
    let col_len = grid[0].len();
    let last_row_i = row_len - 1;
    let last_col_i = col_len - 1;
    let mut q: VecDeque<usize> = VecDeque::with_capacity(row_len * col_len);
    let cord_to_key = |row_i: usize, col_i: usize| -> usize { row_i * col_len + col_i };
    let key_to_cord = |key: usize| -> (usize, usize) { (key / col_len, key % col_len) };
    for (row_i, row) in grid.iter_mut().enumerate() {
        for (col_i, cell) in row.iter_mut().enumerate() {
            if *cell == 1 {
                q.push_back(cord_to_key(row_i, col_i));
                *cell = -1; // Forgot this in the first attempt
            }
        }
    }
    let mut dis = -2;
    while !q.is_empty() {
        let curr_len = q.len();
        for _ in 0..curr_len {
            let curr_key = q.pop_front().expect("Checked length");
            let (row_i, col_i) = key_to_cord(curr_key);

            if row_i > 0 && grid[row_i - 1][col_i] >= 0 {
                q.push_back(cord_to_key(row_i - 1, col_i));
                grid[row_i - 1][col_i] = dis; // Forgot these in the second
            }

            if row_i < last_row_i && grid[row_i + 1][col_i] >= 0 {
                q.push_back(cord_to_key(row_i + 1, col_i));
                grid[row_i + 1][col_i] = dis;
            }

            if col_i > 0 && grid[row_i][col_i - 1] >= 0 {
                q.push_back(cord_to_key(row_i, col_i - 1));
                grid[row_i][col_i - 1] = dis;
            }

            if col_i < last_col_i && grid[row_i][col_i + 1] >= 0 {
                q.push_back(cord_to_key(row_i, col_i + 1));
                grid[row_i][col_i + 1] = dis;
            }
        }
        dis -= 1;
    }
    fn temp_val_to_safety_val(val: i32) -> i32 {
        -val - 1
    }
    // for (row_i, row) in grid.iter().enumerate() {
    //     for (col_i, cell) in row.iter().enumerate() {
    //         print!("{} ", temp_val_to_safety_val(*cell));
    //     }
    //     println!("");
    // }
    let mut visited: Vec<Vec<i32>> = vec![vec![0; col_len]; row_len];
    let start_val = temp_val_to_safety_val(grid[0][0]);
    // visited[0][0] = start_val;
    let mut q: VecDeque<(usize, i32)> = VecDeque::with_capacity(row_len * col_len);
    q.push_back((0, start_val));

    let mut ans_safety = 0;
    while !q.is_empty() {
        let curr_len = q.len();
        for _ in 0..curr_len {
            let (curr_key, prev_val) = q.pop_front().expect("Checked length");
            let (row_i, col_i) = key_to_cord(curr_key);

            let curr_val = temp_val_to_safety_val(grid[row_i][col_i]).min(prev_val);
            // println!("{} {} {}", row_i, col_i, curr_val);
            if row_i == last_row_i && col_i == last_col_i {
                ans_safety = ans_safety.max(curr_val);
            }
            if curr_val <= visited[row_i][col_i] {
                // Forgot the equal sign
                continue;
            }
            visited[row_i][col_i] = curr_val;

            if row_i > 0 {
                q.push_back((cord_to_key(row_i - 1, col_i), curr_val));
            }

            if row_i < last_row_i {
                q.push_back((cord_to_key(row_i + 1, col_i), curr_val));
            }

            if col_i > 0 {
                q.push_back((cord_to_key(row_i, col_i - 1), curr_val));
            }

            if col_i < last_col_i {
                q.push_back((cord_to_key(row_i, col_i + 1), curr_val));
            }
        }
    }
    ans_safety
}
