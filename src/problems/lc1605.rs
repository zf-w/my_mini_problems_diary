//! ## Leetcode 1605. Find Valid Matrix Given Row and Column Sums
//! https://leetcode.com/problems/find-valid-matrix-given-row-and-column-sums/
//! - `Medium`; `Independently Solved`; `2024-07-20`;

pub fn restore_matrix(row_sum: Vec<i32>, col_sum: Vec<i32>) -> Vec<Vec<i32>> {
    let mut row_sum_and_idxs: Vec<(i32, usize)> = Vec::with_capacity(row_sum.len());
    let mut col_sum_and_idxs: Vec<(i32, usize)> = Vec::with_capacity(col_sum.len());

    for (i, row_sum) in row_sum.iter().cloned().enumerate() {
        row_sum_and_idxs.push((row_sum, i));
    }

    for (i, col_sum) in col_sum.iter().cloned().enumerate() {
        col_sum_and_idxs.push((col_sum, i));
    }

    row_sum_and_idxs.sort_unstable();
    col_sum_and_idxs.sort_unstable();

    let mut row_sum_and_idx_iter = row_sum_and_idxs.iter_mut();
    let mut col_sum_and_idx_iter = col_sum_and_idxs.iter_mut();
    let mut curr_row_sum_opt = row_sum_and_idx_iter.next();
    let mut curr_col_sum_opt = col_sum_and_idx_iter.next();

    let mut ans_matrix: Vec<Vec<i32>> = vec![vec![0; col_sum.len()]; row_sum.len()];

    while curr_col_sum_opt.is_some() && curr_row_sum_opt.is_some() {
        let (curr_row_sum, row_i) = curr_row_sum_opt.as_deref_mut().expect("checked");
        let (curr_col_sum, col_i) = curr_col_sum_opt.as_deref_mut().expect("checked");

        if curr_row_sum < curr_col_sum {
            *curr_col_sum -= *curr_row_sum;
            ans_matrix[*row_i][*col_i] = curr_row_sum.clone();
            curr_row_sum_opt = row_sum_and_idx_iter.next();
        } else if curr_col_sum < curr_row_sum {
            *curr_row_sum -= *curr_col_sum;
            ans_matrix[*row_i][*col_i] = curr_col_sum.clone();
            curr_col_sum_opt = col_sum_and_idx_iter.next();
        } else {
            ans_matrix[*row_i][*col_i] = curr_col_sum.clone();
            curr_row_sum_opt = row_sum_and_idx_iter.next();
            curr_col_sum_opt = col_sum_and_idx_iter.next();
        }
    }

    ans_matrix
}
