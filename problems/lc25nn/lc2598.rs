//! # Leetcode 2598. Smallest Missing Non-negative Integer After Operations
//! https://leetcode.com/problems/smallest-missing-non-negative-integer-after-operations/
//! - `Medium`; `y2025m10d16`; `Independently Solved`; `0ms`; `4.1mb`; `1 attempt`;
//! Topics: count_tracking.

pub fn find_smallest_integer(nums: Vec<i32>, value: i32) -> i32 {
    let len = value as usize;
    let mut count_arr_box = vec![0usize; len].into_boxed_slice();

    for num in nums {
        let entry_mut_ref = &mut count_arr_box[(num.rem_euclid(len as i32)) as usize];
        *entry_mut_ref += 1;
    }

    let (fullfill_num, min_fullfill_idx) = count_arr_box
        .into_iter()
        .enumerate()
        .map(|(i, v_ref)| -> (usize, usize) { (v_ref.clone(), i) })
        .min()
        .expect("value > 0");

    (fullfill_num as i32 * value) + min_fullfill_idx as i32
}