//! # Leetcode 1524. Number of Sub-arrays With Odd Sum
//! https://leetcode.com/problems/number-of-sub-arrays-with-odd-sum/
//! - `Medium`; `y2025m02d25`; `Independently Solved`; `0ms`; `3.2mb`; `1 attempt`;
//!
//! Topics: subarray.

const MODULO: i64 = 1000000007;
pub fn num_of_subarrays(arr: Vec<i32>) -> i32 {
    let mut ans_num: i64 = 0;

    let mut prev_even_num: i64 = 0;
    let mut prev_odd_num: i64 = 0;

    for arr_elem in arr {
        (prev_even_num, prev_odd_num) = if arr_elem % 2 == 1 {
            ans_num = (ans_num + prev_even_num + 1) % MODULO;
            (prev_odd_num, (prev_even_num + 1) % MODULO)
        } else {
            ans_num = (ans_num + prev_odd_num) % MODULO;
            ((prev_even_num + 1) % MODULO, prev_odd_num)
        };

        // println!("{} {}", prev_even_num, prev_odd_num);
    }

    ans_num as i32
}
