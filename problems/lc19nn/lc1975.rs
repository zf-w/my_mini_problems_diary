//! # Leetcode 1975. Maximum Matrix Sum
//! https://leetcode.com/problems/maximum-matrix-sum/
//! - `Medium`; `y2024m11d24`; `Hinted`; `2ms` `3.2mb`; `3 attempts`;
//!
//! This problem is also about observation.

pub fn max_matrix_sum(matrix: Vec<Vec<i32>>) -> i64 {
    let mut ans_sum: i64 = 0;
    let mut neg_count: usize = 0;
    let mut min_val_opt: Option<i32> = None;
    for matrix_row_vec in matrix.into_iter() {
        for matrix_entry in matrix_row_vec.into_iter() {
            let curr_val = if matrix_entry <= 0 {
                neg_count += 1;
                -matrix_entry
            } else {
                matrix_entry
            };
            if let Some(min_val_mut_ref) = min_val_opt.as_mut() {
                *min_val_mut_ref = (*min_val_mut_ref).min(curr_val);
            } else {
                min_val_opt = Some(curr_val);
            }
            ans_sum += curr_val as i64;
        }
    }
    if neg_count & 1 == 1 {
        ans_sum - 2 * min_val_opt.expect("Should have a value stored") as i64
    } else {
        ans_sum
    }
}
