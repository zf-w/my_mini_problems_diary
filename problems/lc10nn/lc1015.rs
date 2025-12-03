//! # Leetcode 1015. Smallest Integer Divisible by K
//! https://leetcode.com/problems/smallest-integer-divisible-by-k/
//! - `Medium`; `y2025m11d25`; `Independently Solved`; `0ms`; `2.2mb`; `1 attempt`;
//! Topics: remainder.

pub fn smallest_repunit_div_by_k(k: i32) -> i32 {
    let mut ans_len = 1;
    let mut curr_modulos = 0;

    let mut modulos_map_arr_box = vec![false; k as usize].into_boxed_slice();

    while (curr_modulos * 10 + 1) % k > 0 {
        curr_modulos = (curr_modulos * 10 + 1) % k;

        let modulos_entry_mut_ref = &mut modulos_map_arr_box[curr_modulos as usize];

        if *modulos_entry_mut_ref == true {
            return -1;
        }

        *modulos_entry_mut_ref = true;
        ans_len += 1;
    }

    ans_len
}