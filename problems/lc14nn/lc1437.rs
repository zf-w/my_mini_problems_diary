//! # Leetcode 1437. Check If All 1's Are at Least Length K Places Away
//! https://leetcode.com/problems/check-if-all-1s-are-at-least-length-k-places-away/
//! - `Easy`; `y2025m11d17`; `Independently Solved`; `0ms`; `2.7mb`; `1 attempt`;
//! Topics: count_tracking.

pub fn k_length_apart(nums: Vec<i32>, k: i32) -> bool {
    let k = k as usize;
    nums.into_iter()
        .map(|num| -> bool { num == 1 })
        .fold(
            (None, true),
            |(prev_dis_opt, ans_flag), is_one_flag| -> (Option<usize>, bool) {
                if ans_flag == false {
                    (None, false)
                } else if is_one_flag == true {
                    if let Some(prev_dis) = prev_dis_opt {
                        if prev_dis < k {
                            (None, false)
                        } else {
                            (Some(0), true)
                        }
                    } else {
                        (Some(0), true)
                    }
                } else if let Some(prev_dis) = prev_dis_opt {
                    (Some(prev_dis + 1), true)
                } else {
                    (None, true)
                }
            },
        )
        .1
}