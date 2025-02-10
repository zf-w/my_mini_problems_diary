//! ## Leetcode 3082. Find the Sum of the Power of All Subsequences
//! https://leetcode.com/problems/find-the-sum-of-the-power-of-all-subsequences
//! - `Hard`; `Independently Solved`; `2024-03-16`;
//!
//! This question is the fourth question of Leetcode Biweekly Contest 126.
//!
//! I failed to solve it in time. The intuition is that: given a subsequence's length, we can calculate how many subsequences contain that subsequence. Such that, with a dynamic programming about the length of subsequences of various sums, we can calculate the answer.

pub fn sum_of_power(nums: Vec<i32>, k: i32) -> i32 {
    let k_len: usize = 101;
    let md = 1000000007;
    let mut counts: [[usize; 101]; 101] = [[0; 101]; 101];
    let mut next = counts.clone();
    counts[0][0] = 1;
    let mut ans: usize = 0;
    let n_len = nums.len();

    fn pow2(mut m: usize) -> usize {
        let md = 1000000007;
        let mut multi: usize = 2;
        let mut ans = 1;
        while m > 0 {
            if m % 2 == 1 {
                ans = (ans * multi) % md;
            }
            multi = (multi * multi) % md;
            m /= 2;
        }
        ans
    }

    for num in nums.iter() {
        if k < *num {
            continue;
        }
        let need = k - num;
        let need_i = need as usize;
        // ans = (ans + counts[need_i] as usize) % md;
        for (prev_len, count) in counts[need_i].iter().enumerate() {
            if prev_len < n_len {
                ans = (ans + *count * ((pow2(n_len - prev_len - 1)) as usize % md)) % md;

                // println!("{} {} {} {}", *num, need_i, count, prev_len);
            }
        }

        for i in (*num as usize)..k_len {
            let curr = i as i32;
            let from = curr - num;

            let from_i = from as usize;
            for j in 1..k_len {
                next[i][j] = (counts[from_i][j - 1] + counts[i][j]) % md;
                // if i == 2 && j < n_len {
                //     println!("- {i} {j} {} {}", counts[i][j], counts[from_i][j - 1]);
                // }
            }
        }
        for i in 1..k_len {
            for j in 0..k_len {
                counts[i][j] = next[i][j];
            }
        }
    }

    ans as i32
}

#[test]
fn q4_case_1() {
    let nums = vec![1, 2, 3];
    let k = 3;
    assert_eq!(6, sum_of_power(nums, k));
}

#[test]
fn q4_case_2() {
    let nums = vec![2, 3, 3];
    let k = 5;
    assert_eq!(4, sum_of_power(nums, k));
}

#[test]
fn q4_case_3() {
    let nums = vec![1, 2, 3];
    let k = 7;
    assert_eq!(0, sum_of_power(nums, k));
}

#[test]
fn q4_case_4() {
    let nums = vec![1, 2, 6, 9];
    let k = 8;
    assert_eq!(4, sum_of_power(nums, k));
}
#[test]
fn q4_case_5() {
    let nums = vec![
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    ];
    let k = 25;
    assert_eq!(723214743, sum_of_power(nums, k));
}
