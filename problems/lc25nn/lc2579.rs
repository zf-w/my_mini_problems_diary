//! # Leetcode 2579. Count Total Number of Colored Cells
//! https://leetcode.com/problems/count-total-number-of-colored-cells/
//! - `Medium`; `y2025m03d05`; `Independently Solved`; `0ms`; `2.2mb`; `1 attempt`;
//! Topics: uncategorized.

pub fn colored_cells(n: i32) -> i64 {
    let mut outer_num = 0;
    let mut ans_num = 1;
    for _ in 1..n {
        outer_num += 4;
        ans_num += outer_num;
    }
    ans_num
}
