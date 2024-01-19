//! Leetcode 931. Minimum Falling Path Sum
//! https://leetcode.com/problems/minimum-falling-path-sum
//! - `Medium`; `Independently Solved`; `2024-01-18`;
//!
//! Dynamic Programming with space optimization.

pub fn min_falling_path_sum(matrix: Vec<Vec<i32>>) -> i32 {
    let height = matrix.len();
    let width = matrix[0].len();
    let mut prev: Vec<i32> = matrix[0].clone();
    let mut curr: Vec<i32> = vec![0; width];
    for r in 1..height {
        let row = matrix.get(r).unwrap();
        for (i, v) in curr.iter_mut().enumerate() {
            *v = prev[i];
            if i > 0 {
                *v = (*v).min(prev[i - 1]);
            }
            if i < width - 1 {
                *v = (*v).min(prev[i + 1]);
            }
            *v += row[i];
        }
        std::mem::swap(&mut prev, &mut curr);
    }
    prev.iter().min()
}
