//! ## Leetcode 1652. Defuse the Bomb
//! https://leetcode.com/problems/defuse-the-bomb/
//! - `Easy`; `y2024m11d18`; `Independently Solved`; `0ms`; `2.1mb`; `1 attempt`;
//!

pub fn decrypt(mut code: Vec<i32>, k: i32) -> Vec<i32> {
    if k == 0 {
        for code_elem in code.iter_mut() {
            *code_elem = 0;
        }
        return code;
    }
    let mut sum_i64 = 0;
    for code_elem in code.iter_mut() {
        sum_i64 += *code_elem;
        *code_elem = sum_i64;
    }
    let len = code.len();

    let mut ans_vec: Vec<i32> = Vec::with_capacity(len);

    let calc_fn = |begin_i: usize, end_i: usize| -> i32 {
        let end_i = end_i % len;
        if end_i == 0 {
            return code[len - 1] - code[begin_i - 1];
        }

        if begin_i < end_i {
            let begin_val = if begin_i > 0 { code[begin_i - 1] } else { 0 };
            code[end_i - 1] - begin_val
        } else {
            code[end_i - 1] + code[len - 1] - code[begin_i - 1]
        }
    };

    if k > 0 {
        let k_usize = k as usize;
        for begin_i in 1..=len {
            let end_i = begin_i + k_usize;

            ans_vec.push(calc_fn(begin_i, end_i));
        }
        ans_vec
    } else {
        let k_usize = (-k) as usize;
        for end_i in 0..len {
            let begin_i = (end_i + len - k_usize) % len;
            // println!("{begin_i} {end_i}");
            ans_vec.push(calc_fn(begin_i, end_i));
        }
        ans_vec
    }
}
