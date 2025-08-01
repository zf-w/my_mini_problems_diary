//! # Leetcode 118. ascal's Triangle
//! https://leetcode.com/problems/pascals-triangle/
//! - `Easy`; `y2025m08d01`; `Independently Solved`; `0ms`; `2.1mb`; `1 attempt`;
//! Topics: uncategorized.

pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
    let rows_num = num_rows as usize;
    let mut ans_vec = Vec::with_capacity(rows_num);

    let mut last_row_vec: Vec<i32> = vec![1; 1];

    for i in 1..rows_num {
        let mut curr_row_vec: Vec<i32> = Vec::with_capacity(i + 1);
        curr_row_vec.push(1);
        for curr_i in 1..i {
            curr_row_vec.push(
                last_row_vec[curr_i - 1] + last_row_vec[curr_i],
            );
        }
        curr_row_vec.push(1);

        ans_vec.push(last_row_vec);
        last_row_vec = curr_row_vec;
    }

    ans_vec.push(last_row_vec);

    ans_vec
}