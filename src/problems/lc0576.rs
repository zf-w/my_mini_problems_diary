//! ## Leetcode 576. Out of Boundary Paths
//! https://leetcode.com/problems/out-of-boundary-paths
//! - `Medium`; `Independently Solved`; `2024-01-26`;
//!
//! I tried another way to switch between `prev` and `curr` instead of using memory swaps.
//!

pub fn find_paths(m: i32, n: i32, max_move: i32, start_row: i32, start_column: i32) -> i32 {
    if max_move == 0 {
        return 0;
    }
    let mod_num: i32 = 1000_000_007;
    let mut dp: Vec<i32> = vec![0; 2 * (m * (n + 1)) as usize];
    let height = m as usize;
    let width = n as usize;
    let total = max_move as usize;
    let index = |row: usize, col: usize| -> usize { (row * (width + 1) + col) * 2 };
    for row in 0..height {
        dp[index(row, 0)] += 1;
        dp[index(row, width - 1)] += 1;
    }
    for col in 0..width {
        dp[index(0, col)] += 1;
        dp[index(height - 1, col)] += 1;
    }
    let add = |a: &mut i32, b: i32| {
        *a = (*a + b) % mod_num;
    };
    let target_idx = index(start_row as usize, start_column as usize);
    let mut ans = dp[target_idx];
    let mut prev: usize = 0;
    let mut curr: usize = 1;

    for _ in 2..=total {
        for row in 0..height {
            for col in 0..width {
                let mut curr_val = 0;
                if row > 0 {
                    add(&mut curr_val, dp[index(row - 1, col) + prev]);
                }
                if col > 0 {
                    add(&mut curr_val, dp[index(row, col - 1) + prev]);
                }
                if row < height - 1 {
                    add(&mut curr_val, dp[index(row + 1, col) + prev]);
                }
                if col < width - 1 {
                    add(&mut curr_val, dp[index(row, col + 1) + prev]);
                }
                dp[index(row, col) + curr] = curr_val;
                // print!("{} ", curr_val);
            }
        }
        // println!("");
        prev = (prev + 1) % 2;
        curr = (curr + 1) % 2;
        add(&mut ans, dp[target_idx + prev]);
    }
    ans
}
