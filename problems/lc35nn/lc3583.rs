//! # Leetcode 3583. Count Special Triplets
//! https://leetcode.com/problems/count-special-triplets/
//! - `Medium`; `y2025m12d09`; `Learned from Solution`; `59ms`; `5.3mb`; `1 attempt`;

pub fn special_triplets(nums: Vec<i32>) -> i32 {
            use std::collections::HashMap;

    const MODULOS: i64 = 1_000_000_007;

    let add_with_modulos = |a: i32, b: i32| -> i32 { (((a as i64) + (b as i64)) % MODULOS) as i32 };
    let sub_with_modulos =
        |a: i32, b: i32| -> i32 { (((a as i64) + (MODULOS - (b as i64))) % MODULOS) as i32 };
    let mul_with_modulos = |a: i32, b: i32| -> i32 { (((a as i64) * (b as i64)) % MODULOS) as i32 };

    let mut num_info_hashmap: HashMap<i32, (i32, i32)> = HashMap::with_capacity(nums.len());

    for num in nums.iter().cloned() {
        num_info_hashmap
            .entry(num)
            .and_modify(|(_, total_count_mut_ref)| {
                *total_count_mut_ref += 1;
            })
            .or_insert((0, 1));
    }

    let mut ans_count = 0;

    for num in nums {
        num_info_hashmap
            .entry(num)
            .and_modify(|(left_count_mut_ref, _)| {
                *left_count_mut_ref += 1;
            });

        let target_num = num * 2;

        let (left_count, total_count) =
            num_info_hashmap.get(&target_num).cloned().unwrap_or((0, 0));
        let right_count = sub_with_modulos(total_count, left_count);
        let left_count = if num == 0 {
            left_count - 1
        } else {
            left_count
        };

        ans_count = add_with_modulos(ans_count, mul_with_modulos(left_count, right_count));
    }

    ans_count
}