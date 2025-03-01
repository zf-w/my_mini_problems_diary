//! Leetcode 3097. Shortest Subarray With OR at Least K II
//! https://leetcode.com/problems/shortest-subarray-with-or-at-least-k-ii
//! - `Medium`; `Hinted by Solution`; `2024-03-30`;
//!
//! I spent a lot of time debugging the index function of the prefix sum array.

pub fn minimum_subarray_length(nums: Vec<i32>, k: i32) -> i32 {
    let len = nums.len();
    let max_v = nums.iter().max().unwrap();
    if max_v >= &k {
        return 1;
    }
    let len_1 = len + 1;
    let bits_len = 32;
    let mut dp: Vec<i32> = vec![0; len_1 * bits_len];
    let index = |i: usize, j: usize| -> usize { i * bits_len + j };
    for (i, num) in nums.iter().cloned().enumerate() {
        // let mut bit_i: usize = 0;
        let num = num as u32;
        // println!("{} ", num);
        for bit_i in 0..bits_len {
            dp[index(i + 1, bit_i)] = if num >= 1 << bit_i {
                // print!("->{} ", 1 << bit_i);
                if (num & (1 << bit_i)) > 0 {
                    1
                } else {
                    0
                }
            } else {
                0
            };

            if i > 0 {
                dp[index(i + 1, bit_i)] += dp[index(i, bit_i)];
            }
            // print!("{} ", dp[index(i + 1, bit_i)]);

            // bit_i += 1;
        }
        // println!("");
    }

    let get_or_of_range = |i: usize, j: usize| -> i32 {
        let mut curr: u32 = 0;
        for bit_i in 0..bits_len {
            let idx_j = index(j, bit_i);
            let idx_i = index(i, bit_i);
            // println!("-- {} {} {} {}", i, j, dp[idx_i], dp[idx_j]);
            if dp[idx_j] > dp[idx_i] {
                curr |= 1 << bit_i;
            }
        }
        // println!("{}", curr);
        curr as i32
    };
    let mut begin_i = 0;
    let mut end_i = 0;
    let mut ans = len_1 as i32;
    while end_i < len {
        while end_i < len && get_or_of_range(begin_i, end_i) < k {
            // println!("{}", get_or_of_range(begin_i, end_i));
            end_i += 1;
        }

        while begin_i < end_i && get_or_of_range(begin_i + 1, end_i) >= k {
            // println!("{}", get_or_of_range(begin_i, end_i));
            begin_i += 1;
        }
        if get_or_of_range(begin_i, end_i) >= k {
            ans = ans.min((end_i - begin_i) as i32);
        }
        begin_i += 1;
    }
    if ans > len as i32 {
        -1
    } else {
        ans
    }
}

#[test]
fn check_case_0() {
    let nums = vec![1, 2, 3];
    let k = 2;
    assert_eq!(1, minimum_subarray_length(nums, k))
}

#[test]
fn check_case_1() {
    let nums = vec![2, 1, 8];
    let k = 10;
    assert_eq!(3, minimum_subarray_length(nums, k))
}

#[test]
fn check_case_2() {
    let nums = vec![1, 2, 32, 21];
    let k = 55;
    assert_eq!(3, minimum_subarray_length(nums, k))
}
