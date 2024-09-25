//! ## Leetcode 1380. Lucky Numbers in a Matrix
//! https://leetcode.com/problems/lucky-numbers-in-a-matrix/
//! - `Easy`; `Independently Solved`; `2024-07-19`;

pub fn lucky_numbers(matrix: Vec<Vec<i32>>) -> Vec<i32> {
    let row_len = matrix.len();
    let col_len = matrix[0].len();
    let mut row_min_idxs: Vec<usize> = vec![0; row_len];
    let mut col_max_idxs: Vec<usize> = vec![0; col_len];

    for (row_i, row) in matrix.iter().enumerate() {
        let mut row_iter = row.iter().cloned().enumerate();
        let (mut min_i, mut min_val) = row_iter.next().expect("col_len > 1");
        for (col_i, col_val) in row_iter {
            if col_val < min_val {
                min_i = col_i;
                min_val = col_val;
            }
        }
        row_min_idxs[row_i] = min_i;
    }

    for col_i in 0..col_len {
        let mut row_i = 1;
        let (mut max_i, mut max_val) = (0, matrix[0][col_i]);
        while row_i < row_len {
            let row_val = matrix[row_i][col_i];
            if row_val > max_val {
                max_i = row_i;
                max_val = row_val;
            }
            row_i += 1;
        }
        col_max_idxs[col_i] = max_i;
    }
    let mut ans_vec: Vec<i32> = Vec::new();
    for (row_i, row_min_i) in row_min_idxs.iter().cloned().enumerate() {
        if row_i == col_max_idxs[row_min_i] {
            ans_vec.push(matrix[row_i][row_min_i]);
        }
    }
    ans_vec
}
