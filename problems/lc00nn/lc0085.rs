//! ## Leetcode 85. Maximal Rectangle
//! https://leetcode.com/problems/maximal-rectangle
//! - `Hard`; `Learned from Solution`; `2024-04-13`;
//!
//! Although this question was about using Monotonic Stack, I'm curious about if replacing negative indices with an Option<usize> would be a less-error-prone option.

pub fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
    let col_len = matrix.get(0).expect("Minimum one col").len();
    let mut h = vec![0; col_len];
    let mut l: Vec<Option<usize>> = vec![None; col_len];
    let mut r: Vec<Option<usize>> = vec![None; col_len];
    let mut ans = 0;
    for row in matrix.iter() {
        for (col_i, cell) in row.iter().enumerate() {
            if cell == &'1' {
                h[col_i] += 1;
            } else {
                h[col_i] = 0;
            }
        }

        for col_i in 1..col_len {
            let mut prev_i: Option<usize> = Some(col_i - 1);
            let curr_h = h[col_i];
            while prev_i.is_some() && h[prev_i.unwrap()] >= curr_h {
                prev_i = l[prev_i.unwrap()];
            }
            l[col_i] = prev_i;
        }
        for left_shift in 2..=col_len {
            let col_i = col_len - left_shift;
            let mut next_i: Option<usize> = Some(col_i + 1);
            let curr_h = h[col_i];
            while next_i.is_some() && h[next_i.unwrap()] >= curr_h {
                next_i = r[next_i.unwrap()];
            }
            r[col_i] = next_i;

            let left_i32 = if let Some(left_i) = l[col_i] {
                left_i as i32
            } else {
                -1
            };
            let right_i32 = if let Some(right_i) = next_i {
                right_i as i32
            } else {
                col_len as i32
            };

            ans = ans.max(curr_h * (right_i32 - left_i32 - 1));
        }
    }
    ans
}
