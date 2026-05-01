//! # LeetCode 3488. Closest Equal Element Queries
//! https://leetcode.com/problems/closest-equal-element-queries/
//! - y2026m04d16; Independently Solved;

pub fn solve_queries(nums: Vec<i32>, mut queries: Vec<i32>) -> Vec<i32> {
    use std::collections::HashMap;

    type NumInfo = (usize, [usize; 2]);

    let len = nums.len();

    let mut num_info_hash_map: HashMap<i32, NumInfo> = HashMap::with_capacity(nums.len());

    let calc_circular_len_closure = |idx_0: usize, idx_1: usize| -> i32 {
        let (first_idx, second_idx) = if idx_0 < idx_1 {
            (idx_0, idx_1)
        } else {
            (idx_1, idx_0)
        };

        let len_0 = second_idx - first_idx;

        (len_0).min(len - len_0) as i32
    };

    let mut ans_cheatsheet_i32_vec: Vec<i32> = Vec::with_capacity(len);

    for (i, num) in nums.iter().cloned().enumerate() {
        let (len_mut_ref, arr_mut_ref) = num_info_hash_map.entry(num).or_insert((0, [0, 0]));

        if *len_mut_ref == 0 {
            *len_mut_ref = 1;
            arr_mut_ref[0] = i;
            arr_mut_ref[1] = i;

            ans_cheatsheet_i32_vec.push(-1);
        } else if *len_mut_ref == 1 {
            *len_mut_ref = 2;
            let prev_idx = arr_mut_ref[1];

            let curr_ans = calc_circular_len_closure(arr_mut_ref[0], i);

            ans_cheatsheet_i32_vec[prev_idx] = curr_ans;
            ans_cheatsheet_i32_vec.push(curr_ans);

            arr_mut_ref[1] = i;
        } else {
            let first_idx = arr_mut_ref[0];
            let prev_idx = arr_mut_ref[1];

            let first_num_candidate = calc_circular_len_closure(first_idx, i);
            let prev_num_candidate = calc_circular_len_closure(prev_idx, i);

            ans_cheatsheet_i32_vec[first_idx] =
                ans_cheatsheet_i32_vec[first_idx].min(first_num_candidate);
            ans_cheatsheet_i32_vec[prev_idx] =
                ans_cheatsheet_i32_vec[prev_idx].min(prev_num_candidate);
            ans_cheatsheet_i32_vec.push(first_num_candidate.min(prev_num_candidate));

            arr_mut_ref[1] = i;
        }
    }

    for query_mut_ref in queries.iter_mut() {
        let num_i = *query_mut_ref as usize;
        *query_mut_ref = ans_cheatsheet_i32_vec[num_i];
    }

    queries
}
