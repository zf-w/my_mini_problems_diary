//! ## Leetcode 1861. Rotating the Box
//! https://leetcode.com/problems/rotating-the-box/
//! - `Medium`; `y2024m11d23`; `Independently Solved`; `142ms`; `19mb`; `1 attempt`;
//!
//! I guess this is not cache-friendly.

pub fn rotate_the_box(mut boxes_vec: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let row_len = boxes_vec.len();
    let col_len = boxes_vec[0].len();

    let mut empty_i_queue: std::collections::VecDeque<usize> =
        std::collections::VecDeque::with_capacity(col_len);

    for box_row_vec_mut_ref in boxes_vec.iter_mut() {
        let col_len = box_row_vec_mut_ref.len();
        let row_last_i = col_len - 1;

        empty_i_queue.clear();
        for col_i_off in 0..col_len {
            let col_i = row_last_i - col_i_off;
            let curr_box_char = box_row_vec_mut_ref[col_i];
            if curr_box_char == '*' {
                empty_i_queue.clear();
            } else if curr_box_char == '.' {
                empty_i_queue.push_back(col_i);
            } else if let Some(empty_space_i) = empty_i_queue.pop_front() {
                box_row_vec_mut_ref[empty_space_i] = '#';
                box_row_vec_mut_ref[col_i] = '.';
                empty_i_queue.push_back(col_i);
            }
        }
    }
    let mut ans_vec: Vec<Vec<char>> = Vec::with_capacity(col_len);
    for col_i in 0..col_len {
        let mut ans_col_vec = Vec::with_capacity(row_len);
        for row_ref in boxes_vec.iter().rev() {
            ans_col_vec.push(row_ref[col_i]);
        }
        ans_vec.push(ans_col_vec);
    }
    // ans_vec.reverse();
    ans_vec
}
