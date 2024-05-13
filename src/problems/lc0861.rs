//! ## Leetcode 861. Scpre After Flipping Matrix
//! https://leetcode.com/problems/score-after-flipping-matrix
//! - `Medium`; `Learned from Solution`; `2024-05-12`;
//!
//! Learned from: https://leetcode.com/problems/score-after-flipping-matrix/solutions/5149964/fastest-100-easy-to-understand-clean-concise
//! Not sure why this greedy solution works. I get that making the first column one and tuning the rest of the bits is reasonable. But why that solution works on all the cases is interesting to think about.

pub fn matrix_score(grid: Vec<Vec<i32>>) -> i32 {
    let row_len = grid.len();
    let col_len = grid[0].len(); // Should work, minimum 1 row.
    let mut ans = (1 << (col_len - 1)) * row_len;
    for col_i in 1..col_len {
        let mut count = 0;
        let val = 1 << (col_len - 1 - col_i);
        for row_i in 0..row_len {
            if grid[row_i][col_i] == grid[row_i][0] {
                count += 1;
            }
        }

        ans += count.max(row_len - count) * val;
    }
    ans as i32
}
