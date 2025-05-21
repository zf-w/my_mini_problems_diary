//! # Leetcode 73. Set Matrix Zeroes
//! https://leetcode.com/problems/set-matrix-zeroes/
//! - `Medium`; `y2025m05d20`; `Independently Solved`; `0ms`; `2.5mb`; `1 attempt`;
//! Topics: uncategorized.

pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
    let row_num = matrix.len();
    let col_num = matrix[0].len();
    let mut row_and_col_flag_box_arr: Box<[bool]> =
        vec![false; row_num + col_num].into_boxed_slice();

    for (row_i, row_vec_ref) in matrix.iter().enumerate() {
        for (col_i, entry_ref) in row_vec_ref.iter().enumerate() {
            if *entry_ref != 0 {
                continue;
            }

            row_and_col_flag_box_arr[row_i] = true;
            row_and_col_flag_box_arr[row_num + col_i] = true;
        }
    }

    for (row_i, row_to_mut_flag) in row_and_col_flag_box_arr[..row_num]
        .iter()
        .cloned()
        .enumerate()
    {
        if row_to_mut_flag == false {
            continue;
        }

        for entry_mut_ref in matrix[row_i].iter_mut() {
            *entry_mut_ref = 0;
        }
    }

    for (col_i, col_to_mut_flag) in row_and_col_flag_box_arr[row_num..]
        .iter()
        .cloned()
        .enumerate()
    {
        if col_to_mut_flag == false {
            continue;
        }

        for row_mut_ref in matrix.iter_mut() {
            row_mut_ref[col_i] = 0;
        }
    }
}
