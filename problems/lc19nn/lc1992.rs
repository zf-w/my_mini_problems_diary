//! ## Leetcode 1992. Find All Groups of Farmland
//! https://leetcode.com/problems/find-all-groups-of-farmland
//! - `Medium`; `Learned from Solution`; `2024-04-20`;
//!
//! At first, I was not understanding the term "rectangular" implies a group is always rectangular and valid.

pub fn find_farmland(land: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut ans: Vec<Vec<i32>> = Vec::new();
    let row_len = land.len();
    let col_len = land[0].len();
    for row_i in 0..row_len {
        for col_i in 0..col_len {
            let curr = land[row_i][col_i];
            if curr == 1
                && (row_i == 0 || land[row_i - 1][col_i] == 0)
                && (col_i == 0 || land[row_i][col_i - 1] == 0)
            {
                let mut next_row_i = row_i;
                while next_row_i + 1 < row_len && land[next_row_i + 1][col_i] == 1 {
                    next_row_i += 1;
                }

                let mut next_col_i = col_i;
                while next_col_i + 1 < col_len && land[row_i][next_col_i + 1] == 1 {
                    next_col_i += 1;
                }
                ans.push(vec![
                    row_i as i32,
                    col_i as i32,
                    next_row_i as i32,
                    next_col_i as i32,
                ]);
            }
        }
    }
    ans
}
