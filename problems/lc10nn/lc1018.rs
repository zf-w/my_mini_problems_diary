//! # Leetcode 1018. Binary Prefix Divisible By 5
//! https://leetcode.com/problems/binary-prefix-divisible-by-5/
//! - `Easy`; `y2025m11d24`; `Independently Solved`; `0ms`; `2.4mb`; `1 attempt`;
//! Topics: remainder.

pub fn prefixes_div_by5(nums: Vec<i32>) -> Vec<bool> {
    let len = nums.len();
    nums.into_iter()
        .fold(
            (Vec::with_capacity(len), 0),
            |(mut ans_vec, prev_modulos), num| -> (Vec<bool>, i32) {
                let curr_modulos = ((prev_modulos << 1) + num) % 5;
                if curr_modulos == 0 {
                    ans_vec.push(true);
                } else {
                    ans_vec.push(false);
                }
                (ans_vec, curr_modulos)
            },
        )
        .0
}