//! # Leetcode 1072. Flip Columns For Maximum Number of Equal Rows
//! https://leetcode.com/problems/flip-columns-for-maximum-number-of-equal-rows/
//! - `Medium`; `y2024m11d22`; `Independently Solved`; `26ms`; `2.8mb`; `1 attempt`;
//!
//! I was going to decide to copy from a solution at first, but then I saw the size of the matrix was quite small and decided to try it myself instead.
//!

pub fn max_equal_rows_after_flips(matrix: Vec<Vec<i32>>) -> i32 {
    fn cmp_if_same_group(row_0_ref: &[i32], row_1_ref: &[i32]) -> bool {
        let mut all_same: bool = true;
        let mut all_flip: bool = true;
        for (elem_0, elem_1) in row_0_ref.iter().cloned().zip(row_1_ref.iter().cloned()) {
            if elem_0 == elem_1 {
                all_flip = false;
            } else {
                all_same = false;
            }
        }
        all_same || all_flip
    }

    let mut row_vec_ref_iter = matrix.iter().enumerate();
    let len = matrix.len();
    let mut checked_flag_vec: Vec<bool> = vec![false; len];

    let mut ans_max_group_count: usize = 0;

    while let Some((row_i, row_vec_ref)) = row_vec_ref_iter.next() {
        if checked_flag_vec[row_i] == true {
            continue;
        }
        let mut group_count: usize = 1;
        checked_flag_vec[row_i] = true;
        let row_vec_ref_iter_1 = row_vec_ref_iter.clone();
        for (row_1_i, row_1_vec_ref) in row_vec_ref_iter_1 {
            if cmp_if_same_group(row_vec_ref.as_slice(), row_1_vec_ref.as_slice()) {
                checked_flag_vec[row_1_i] = true;
                group_count += 1;
            }
        }
        ans_max_group_count = ans_max_group_count.max(group_count);
    }
    ans_max_group_count as i32
}
