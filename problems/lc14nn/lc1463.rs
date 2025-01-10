//! ## Leetcode 1463. Cherry Pickup II
//! https://leetcode.com/problems/cherry-pickup-ii
//! - `Hard`; `Independently Solved`; `2024-02-11`;
//!
//! A comprehensive question that allows me to apply my designs: `prev` `curr` DP switching with an offset instead of pointers and a "combination" DP instead of a "permutation" DP.

pub fn cherry_pickup(grid: Vec<Vec<i32>>) -> i32 {
    let w = grid[0].len();
    let last = w - 1;
    let dp: &mut [i32] = &mut vec![0; w * w + w];
    let index = |i: usize, j: usize, off: bool| -> usize {
        ((j + j * j) / 2 + i) * 2 + if off { 0 } else { 1 }
    };
    let mut prev = false;
    let mut curr = true;
    let h = grid.len();
    for back in 1..=h {
        let level = h - back;
        for j in 0..w {
            // Bug 1: should be inclusive here
            for i in 0..=j {
                // Bug 1: should be inclusive here
                let mut curr_dp = 0;
                let prev_j_min = if j > 0 { j - 1 } else { 0 };
                let prev_j_max = if j < last { j + 1 } else { last };
                let prev_i_min = if i > 0 { i - 1 } else { 0 }; // Bug 2.1: Should be i - 1 here. Sometimes, two bugs hide one another. Didn't reveal itself before bug 2.0. Passed case 1 even without fixing bugs. However, case 1 will not pass if you fixed bug 2.0.
                                                                // let prev_i_max = if i < j { i + 1 } else { j };

                for prev_j in prev_j_min..=prev_j_max {
                    let prev_i_max = if i < prev_j { i + 1 } else { prev_j }; // Bug 2.0: `i` can't be `prev_j_max` if prev_j was from the left. `i` should be smaller than or equal to `j` at all times.
                    for prev_i in prev_i_min..=prev_i_max {
                        curr_dp = curr_dp.max(dp[index(prev_i, prev_j, prev)]);
                        if i == 2 && j == 3 && level == 2 {
                            // println!("{}, {}", prev_i, prev_j);
                        }
                    }
                }
                if i != j {
                    curr_dp += grid[level][i] + grid[level][j];
                } else {
                    curr_dp += grid[level][i];
                }
                dp[index(i, j, curr)] = curr_dp;
                // print!("({}, {}, {})", i, j, curr_dp);
            }
        }
        // println!("");
        prev = !prev;
        curr = !curr;
    }

    dp[index(0, w - 1, prev)]
}

#[test]
fn check_case1() {
    let grid = vec![vec![3, 1, 1], vec![2, 5, 1], vec![1, 5, 5], vec![2, 1, 1]]; //,, ;
    assert_eq!(24, cherry_pickup(grid));
}

#[test]
fn check_case2() {
    let grid = vec![
        vec![1, 0, 0, 0, 0, 0, 1],
        vec![2, 0, 0, 0, 0, 3, 0],
        vec![2, 0, 9, 0, 0, 0, 0],
        vec![0, 3, 0, 5, 4, 0, 0],
        vec![1, 0, 2, 3, 0, 0, 6],
    ];
    assert_eq!(28, cherry_pickup(grid));
}

/// ## Key debugging message (WA: 25)
///
/// (0, 0, 9)(0, 1, 9)(1, 1, 0)(0, 2, 12)(1, 2, 3)(2, 2, 3)(0, 3, 12)(1, 3, 3)(2, 3, 6)(3, 3, 3)
///
/// (0, 0, 9)(0, 1, 12)(1, 1, 12)(0, 2, 15)(1, 2, 15)(2, 2, 15)(0, 3, 15)(1, 3, 15)1, 2
///
/// 2, 2
///
/// 3, 2
///
/// 1, 3
///
/// 2, 3
///
/// 3, 3
///
/// (2, 3, 18)(3, 3, 15)
///
/// (0, 0, 12)(0, 1, 15)(1, 1, 15)(0, 2, 15)(1, 2, 18)(2, 2, 18)(0, 3, 18)(1, 3, 21)(2, 3, 21)(3, 3, 21)
///
/// (0, 0, 16)(0, 1, 19)(1, 1, 18)(0, 2, 22)(1, 2, 21)(2, 2, 21)(0, 3, 25)(1, 3, 24)(2, 3, 24)(3, 3, 24)
#[test]
fn check_case3() {
    let grid = vec![
        vec![1, 0, 0, 3],
        vec![0, 0, 0, 3],
        vec![0, 0, 3, 3],
        vec![9, 0, 3, 3],
    ];
    assert_eq!(22, cherry_pickup(grid));
}
