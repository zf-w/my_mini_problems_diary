//! # Leetcode 3381. Maximum Subarray Sum With Length Divisible by K
//! https://leetcode.com/problems/maximum-subarray-sum-with-length-divisible-by-k/
//! - `Medium`; `y2025m11d27`; `Learned from Solution`; `3ms`; `6mb`; `1 attempt`;
//! Topics: remainder.
//! https://leetcode.com/problems/maximum-subarray-sum-with-length-divisible-by-k/editorial

pub fn max_subarray_sum(nums: Vec<i32>, k: i32) -> i64 {
    let k_len = k as usize;
    let mut min_sum_at_modulos_arr_box: Box<[Option<i64>]> = vec![None; k_len].into_boxed_slice();

    min_sum_at_modulos_arr_box[k_len - 1] = Some(0); // "Enables" subarray starting at the zeroth index.

    let mut curr_sum: i64 = 0;
    let mut ans_max_sum = i64::MIN;

    for (i, num) in nums.into_iter().enumerate() {
        curr_sum += num as i64;
        let modulos_index = i % k_len;

        let min_sum_at_modulos_entry_mut_ref = &mut min_sum_at_modulos_arr_box[modulos_index];

        if let Some(min_sum_at_modulos_mut_ref) = min_sum_at_modulos_entry_mut_ref {
            
            ans_max_sum = ans_max_sum.max(curr_sum - *min_sum_at_modulos_mut_ref);
            
            *min_sum_at_modulos_mut_ref = (*min_sum_at_modulos_mut_ref).min(curr_sum);
        } else {
            *min_sum_at_modulos_entry_mut_ref = Some(curr_sum);
        }
    }

    ans_max_sum
}