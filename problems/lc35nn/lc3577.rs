//! # Leetcode 3577. Count the Number of Computer Unlocking Permutations
//! https://leetcode.com/problems/count-the-number-of-computer-unlocking-permutations/
//! - `Medium`; `y2025m12d09`; `Independently Solved`; `0ms`; `3.54mb`; `1 attempt`;

pub fn count_permutations(complexity_vec: Vec<i32>) -> i32 {
    const MODULOS: i64 = 1_000_000_007;

    let mul_with_modulos = |a: i32, b: i32| -> i32 { (((a as i64) * (b as i64)) % MODULOS) as i32 };

    let base_complexity = complexity_vec[0];
    let mut ans_count = 1;
    let mut curr_multiplier = 1;

    for complexity in complexity_vec.iter().cloned().skip(1) {
        if complexity == base_complexity {
            return 0;
        }

        ans_count = mul_with_modulos(ans_count, curr_multiplier);
        curr_multiplier += 1;
    }

    ans_count
}