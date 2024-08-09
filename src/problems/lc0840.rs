//! ## Leetcode 840. Magic Squares In Grid
//! https://leetcode.com/problems/magic-squares-in-grid/
//! - `Medium`; `Independently Solved`; `2024-08-09`;

fn check_is_magic_square(row_0_ref: &[i32], row_1_ref: &[i32], row_2_ref: &[i32]) -> bool {
    if row_0_ref[0] + row_0_ref[1] + row_0_ref[2] != 15
        || row_1_ref[0] + row_1_ref[1] + row_1_ref[2] != 15
        || row_2_ref[0] + row_2_ref[1] + row_2_ref[2] != 15
        || row_0_ref[0] + row_1_ref[0] + row_2_ref[0] != 15
        || row_0_ref[1] + row_1_ref[1] + row_2_ref[1] != 15
        || row_0_ref[2] + row_1_ref[2] + row_2_ref[2] != 15
        || row_0_ref[0] + row_1_ref[1] + row_2_ref[2] != 15
        || row_0_ref[2] + row_1_ref[1] + row_2_ref[0] != 15
    {
        return false;
    }
    let mut bit_flag: u16 = 0;
    for num in row_0_ref.iter().cloned() {
        if num < 10 && num > 0 {
            bit_flag |= 1 << (num - 1);
        }
    }
    for num in row_1_ref.iter().cloned() {
        if num < 10 && num > 0 {
            bit_flag |= 1 << (num - 1);
        }
    }
    for num in row_2_ref.iter().cloned() {
        if num < 10 && num > 0 {
            bit_flag |= 1 << (num - 1);
        }
    }
    // println!("{} {}", bit_flag, (1 << 10 - 1) - 1);
    bit_flag == (1 << 10 - 1) - 1
}

pub fn num_magic_squares_inside(grid: Vec<Vec<i32>>) -> i32 {
    let row_num = grid.len();
    let col_num = grid[0].len();
    if row_num < 3 || col_num < 3 {
        return 0;
    }
    let mut ans_count = 0;
    for row_i in 0..(row_num - 2) {
        for col_i in 0..(col_num - 2) {
            let row_0_ref = &grid[row_i][col_i..(col_i + 3)];
            let row_1_ref = &grid[row_i + 1][col_i..(col_i + 3)];
            let row_2_ref = &grid[row_i + 2][col_i..(col_i + 3)];
            if check_is_magic_square(row_0_ref, row_1_ref, row_2_ref) {
                ans_count += 1;
            }
        }
    }
    ans_count
}
