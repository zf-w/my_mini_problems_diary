//! # Leetcode 2948. Make Lexicographically Smallest Array by Swapping Elements
//! https://leetcode.com/problems/make-lexicographically-smallest-array-by-swapping-elements/
//! - `Medium`; `y2025m01d25`; `Independently Solved`; `31ms`; `4.3mb`; `1 attempt`;

pub fn lexicographically_smallest_array(mut nums: Vec<i32>, limit: i32) -> Vec<i32> {
    let len = nums.len();
    let mut idx_vec: Vec<usize> = (0..len).collect();

    idx_vec.sort_unstable_by_key(|idx_ref| -> i32 { nums[*idx_ref] });

    nums.sort_unstable();

    let mut group_begin_i: usize = 0;

    fn rearrange_nums_fn(
        group_idx_arr_mut_ref: &mut [usize],
        nums_group_ref: &[i32],
        ans_nums_mut_ref: &mut [i32],
    ) {
        group_idx_arr_mut_ref.sort_unstable();
        for (from_idx, to_idx) in group_idx_arr_mut_ref.iter().enumerate() {
            ans_nums_mut_ref[*to_idx] = nums_group_ref[from_idx];
        }
    }

    let mut ans_num_vec: Vec<i32> = vec![0; len];

    for next_i in 1..len {
        let curr_i = next_i - 1;
        if nums[next_i] - nums[curr_i] > limit {
            rearrange_nums_fn(
                &mut idx_vec[group_begin_i..next_i],
                &nums[group_begin_i..next_i],
                &mut ans_num_vec,
            );
            group_begin_i = next_i;
        }
    }

    rearrange_nums_fn(
        &mut idx_vec[group_begin_i..],
        &nums[group_begin_i..],
        &mut ans_num_vec,
    );

    ans_num_vec
}
