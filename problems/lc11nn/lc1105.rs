//! ## Leetcode 1105. Filling Bookcase Shelves
//! https://leetcode.com/problems/filling-bookcase-shelves/
//! - `Medium`; `Independently Solved`; `2024-07-31`;
//!
pub fn min_height_shelves(books: Vec<Vec<i32>>, shelf_width: i32) -> i32 {
    let mut min_height_dp_vec: Vec<i32> = Vec::with_capacity(books.len());
    let mut ans_min_height = 0;
    for (curr_book_i, book_ref) in books.iter().enumerate() {
        let mut curr_max_height = book_ref[1];
        let mut curr_width = book_ref[0];
        if curr_book_i == 0 {
            ans_min_height = curr_max_height;
        } else {
            ans_min_height = i32::MAX;
        }
        for (prev_book_ref, prev_book_dp) in books[..curr_book_i]
            .iter()
            .rev()
            .zip(min_height_dp_vec.iter().rev().cloned())
        {
            let prev_book_width = prev_book_ref[0];
            let prev_book_height = prev_book_ref[1];
            curr_width += prev_book_width;
            ans_min_height = ans_min_height.min(prev_book_dp + curr_max_height);
            if curr_width > shelf_width {
                break;
            }
            curr_max_height = prev_book_height.max(curr_max_height);
        }
        if curr_width <= shelf_width {
            ans_min_height = ans_min_height.min(curr_max_height);
        }
        min_height_dp_vec.push(ans_min_height);
    }
    ans_min_height
}
