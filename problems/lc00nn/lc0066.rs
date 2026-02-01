//! # Leetcode 66. Plus One
//! https://leetcode.com/problems/plus-one/
//! - `Easy`; `y2026m01d01`; `Independently Solved`; `0ms`; `2.2mb`; `1 attempt`;

pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
    let mut to_add = 1;
    let mut ans_vec: Vec<i32> = Vec::with_capacity(digits.len() + 1);

    for digit in digits.into_iter().rev() {
        let curr_total = digit + to_add;
        to_add = curr_total / 10;
        ans_vec.push(curr_total % 10);
    }

    if to_add > 0 {
        ans_vec.push(to_add);
    }

    let ans_len = ans_vec.len();

    for i in 0..(ans_len / 2) {
        let rev_i = ans_len - 1 - i;
        let temp = ans_vec[i];
        ans_vec[i] = ans_vec[rev_i];
        ans_vec[rev_i] = temp;
    }

    ans_vec
}