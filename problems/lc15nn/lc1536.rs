//! # Leetcode 1536. Minimum Swaps to Arrange a Binary Grid
//! https://leetcode.com/problems/minimum-swaps-to-arrange-a-binary-grid/
//! - y2026m03d02; Independently Solved;

pub fn min_swaps(grid: Vec<Vec<i32>>) -> i32 {
    let len = grid.len();
    let mut zero_count_vec = Vec::with_capacity(len);
    grid.iter().for_each(|row_vec_mut_ref| {
        let mut zero_count: i32 = 0;
        for num in row_vec_mut_ref.iter().rev().cloned() {
            if num == 0 {
                zero_count += 1;
            } else {
                break;
            }
        }

        zero_count_vec.push(zero_count);
    });

    let mut ans_count = 0;

    for i in 0..len {
        let need_zero_count = (len - i - 1) as i32;
        let mut found_idx_opt = None;
        for j in i..len {
            let zero_count = zero_count_vec[j];
            if zero_count >= need_zero_count {
                // zero_count_vec.swap(i, j);
                ans_count += (j - i) as i32;
                found_idx_opt = Some(j);
                break;
            }
        }

        if let Some(idx) = found_idx_opt {
            for begin_i in (i..idx).rev() {
                zero_count_vec.swap(begin_i, begin_i + 1);
            }
        } else {
            return -1;
        }
    }

    ans_count
}
