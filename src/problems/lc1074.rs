//! ## Leetcode 1074. Number of Submatrices That Sum to Target
//! https://leetcode.com/problems/number-of-submatrices-that-sum-to-target
//! - `Hard`; `Learned from Solution`; `2024-01-27`;
//!
//! The 2D prefix sum is indeed interesting. Although I don't need the exact 2D prefix sum, a 1D array of 1D prefix sums would also work. The most tricky bug of today would be mistyping `w` into `h.` I guess there is indeed some merit in naming variables somewhat long.

use std::collections::HashMap;

pub fn num_submatrix_sum_target(matrix: Vec<Vec<i32>>, target: i32) -> i32 {
    let h = matrix.len();
    let w = matrix[0].len();
    fn get_prefix_sum(m: &Vec<Vec<i32>>) -> Vec<i32> {
        let h = m.len();
        let w = m[0].len();
        let mut ans = vec![0; h * w];
        let index = |i, j| -> usize { i * w + j };
        ans[0] = m[0][0];
        for i in 1..h {
            ans[index(i, 0)] = ans[index(i - 1, 0)] + m[i][0];
        }
        for j in 1..w {
            ans[index(0, j)] = ans[index(0, j - 1)] + m[0][j];
        }
        for i in 1..h {
            for j in 1..w {
                ans[index(i, j)] = ans[index(i - 1, j)] + ans[index(i, j - 1)]
                    - ans[index(i - 1, j - 1)]
                    + m[i][j];
                // print!("{}", ans[index(i, j)]);
            }
            // println!("");
        }
        ans
    }
    let pre = get_prefix_sum(&matrix);
    let index = |i, j| -> usize { i * w + j };
    let get_sub_sum = |i: usize, j: usize, i1: usize, j1: usize| -> i32 {
        pre[index(i1, j1)]
            - if i > 0 { pre[index(i - 1, j1)] } else { 0 }
            - if j > 0 { pre[index(i1, j - 1)] } else { 0 }
            + if i > 0 && j > 0 {
                pre[index(i - 1, j - 1)]
            } else {
                0
            }
    };
    // println!("{}",get_sub_sum(2,0,2,1));
    let mut res: i32 = 0;
    for i in 0..h {
        for i1 in i..h {
            let mut count: HashMap<i32, i32> = HashMap::new();
            count.insert(0, 1);
            for j1 in 0..w {
                let curr_sum = get_sub_sum(i, 0, i1, j1);
                // println!("({i} 0) ({i1}, {j1}) -> {curr_sum}");
                res += count.get(&(curr_sum - target)).unwrap_or(&0);
                if let Some(v) = count.get_mut(&curr_sum) {
                    *v += 1;
                } else {
                    count.insert(curr_sum, 1);
                }
            }
        }
    }
    res
}
