//! ## Leetcode 2134. Minimum Swaps to Group All 1's Together II
//! https://leetcode.com/problems/minimum-swaps-to-group-all-1s-together-ii/
//! - `Medium`; `Independently Solved`; `2024-08-01`;

pub fn min_swaps(mut nums: Vec<i32>) -> i32 {
    let len = nums.len();
    let mut ans_min_count: Option<usize> = None;
    let mut prev_sum = 0;
    let mut one_count: usize = 0;
    for num_mut_ref in nums.iter_mut() {
        if *num_mut_ref == 1 {
            one_count += 1;
        }
        prev_sum += *num_mut_ref;
        *num_mut_ref = prev_sum;
    }
    let get_one_count_within_range = |begin_i: usize, end_i: usize| -> usize {
        ((if begin_i == 0 { 0 } else { nums[begin_i - 1] })
            - (if end_i == 0 { 0 } else { nums[end_i - 1] })) as usize
    };
    for begin_i in 0..len {
        let end_i = begin_i + one_count;
        let curr_one_count = if end_i > len {
            let part_1_end_i = len - end_i - 1;
            get_one_count_within_range(0, part_1_end_i) + get_one_count_within_range(begin_i, len)
        } else {
            get_one_count_within_range(begin_i, end_i)
        };
        if let Some(v_mut_ref) = ans_min_count.as_mut() {
            *v_mut_ref = (*v_mut_ref).min(one_count - curr_one_count);
        } else {
            ans_min_count = Some(one_count - curr_one_count);
        }
    }
    ans_min_count.expect("Should have") as i32
}
