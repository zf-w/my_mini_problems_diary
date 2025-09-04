//! # Leetcode 498. Diagonal Traverse
//! https://leetcode.com/problems/diagonal-traverse/
//! - `Medium`; `y2025m08d26`; `Independently Solved`; `0ms`; `2.6mb`; `1 attempt`;
//! Topics: uncategorized.

pub fn find_diagonal_order(mat: Vec<Vec<i32>>) -> Vec<i32> {
    let mut row_i = 0;
    let mut col_i = 0;
    let mut is_going_up_flag: bool = true;

    let row_num = mat.len();
    let col_num = mat[0].len();
    let total_cell_num = row_num * col_num;

    let mut ans_vec = Vec::with_capacity(total_cell_num);

    while ans_vec.len() < total_cell_num {
        ans_vec.push(mat[row_i][col_i]);

        if is_going_up_flag == true {
            if row_i == 0 {
                is_going_up_flag = !is_going_up_flag;
            } else {
                row_i -= 1;
            }
            if col_i + 1 < col_num {
                col_i += 1;
            } else {
                row_i -= 1;
            }
        } else {
            if col_i == 0 {
                is_going_up_flag = !is_going_up_flag;
            } else {
                col_i -= 1;
            }
            if row_i + 1 < row_num {
                row_i += 1;
            } else {
                col_i += 1;
            }
        }
    }

    ans_vec
}
