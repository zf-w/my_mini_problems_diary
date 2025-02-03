//! # Leetcode 2661. First Completely Painted Row or Column
//! https://leetcode.com/problems/first-completely-painted-row-or-column/
//! - `Medium`; `y2025m01d20`; `Independently Solved`; `0ms`; `7mb`; `1 attempt`;

pub fn first_complete_index(arr: Vec<i32>, mat: Vec<Vec<i32>>) -> i32 {
    let row_num = mat.len();
    let col_num = mat[0].len();
    let mut position_vec: Vec<usize> = vec![0; row_num * col_num];

    let mut row_paint_count: Vec<usize> = vec![0; row_num];
    let mut col_paint_count: Vec<usize> = vec![0; col_num];

    for (row_i, row_ref) in mat.iter().enumerate() {
        for (col_i, cell_ref) in row_ref.iter().enumerate() {
            let cell_idx = (*cell_ref) as usize;
            position_vec[cell_idx] = row_i * col_num + col_i;
        }
    }

    for (i, to_paint_idx) in arr.into_iter().map(|v| -> usize { v as usize }).enumerate() {
        let position = position_vec[to_paint_idx];
        let row_i = position / col_num;
        let col_i = position % col_num;

        let row_paint_mut_ref = row_paint_count
            .get_mut(row_i)
            .expect("getting paint count mut ref, expecting within len");

        *row_paint_mut_ref += 1;
        if *row_paint_mut_ref == col_num {
            return i as i32;
        }

        let col_paint_mut_ref = col_paint_count
            .get_mut(col_i)
            .expect("getting paint count mut ref, expecting within len");

        *col_paint_mut_ref += 1;
        if *col_paint_mut_ref == row_num {
            return i as i32;
        }
    }
    0
}
