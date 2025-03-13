//! # Leetcode 3356. Zero Array Transformation II
//! https://leetcode.com/problems/zero-array-transformation-ii/
//! - `Medium`; `y2025m03d12`; `Learned from Solution`; `8ms`; `9.7mb`; `3 attempts`;
//! Topics: binary_search,difference_array.

pub fn min_zero_array(mut nums: Vec<i32>, queries: Vec<Vec<i32>>) -> i32 {
    let len = nums.len();

    let mut diff_vec: Vec<i32> = vec![0; len + 1];
    let mut begin_query_end_i: usize = 0;
    let mut end_query_end_i: usize = queries.len() + 1;

    fn update_diff_arr(
        diff_arr_mut_ref: &mut [i32],
        prev_query_end_i: usize,
        curr_query_end_i: usize,
        queries_arr_ref: &[Vec<i32>],
    ) {
        if curr_query_end_i > prev_query_end_i {
            for query_vec in queries_arr_ref
                .iter()
                .take(curr_query_end_i)
                .skip(prev_query_end_i)
            {
                let begin_i = query_vec[0] as usize;
                let end_i = query_vec[1] as usize + 1;
                let dec_max = query_vec[2];
                diff_arr_mut_ref[begin_i] += dec_max;
                diff_arr_mut_ref[end_i] -= dec_max;
            }
        } else {
            for query_vec in queries_arr_ref
                .iter()
                .take(prev_query_end_i)
                .skip(curr_query_end_i)
            {
                let begin_i = query_vec[0] as usize;
                let end_i = query_vec[1] as usize + 1;
                let dec_max = query_vec[2];
                diff_arr_mut_ref[begin_i] -= dec_max;
                diff_arr_mut_ref[end_i] += dec_max;
            }
        }
    }

    let tell_diff_arr_ok = |diff_arr_ref: &[i32]| {
        let mut curr_prefix_sum = 0;
        for (diff_elem_ref, num_ref) in diff_arr_ref.iter().zip(nums.iter()) {
            curr_prefix_sum += *diff_elem_ref;
            if curr_prefix_sum < *num_ref {
                return false;
            }
        }
        true
    };

    let mut prev_query_end_i: usize = 0;
    let mut found_flag = false;

    while begin_query_end_i < end_query_end_i {
        let mid_query_end_i = (begin_query_end_i + end_query_end_i) / 2;
        update_diff_arr(&mut diff_vec, prev_query_end_i, mid_query_end_i, &queries);
        // println!("{}", mid_query_end_i);
        if tell_diff_arr_ok(&diff_vec) {
            end_query_end_i = mid_query_end_i;
            found_flag = true;
        } else {
            begin_query_end_i = mid_query_end_i + 1;
        }
        prev_query_end_i = mid_query_end_i;
    }
    if found_flag {
        begin_query_end_i as i32
    } else {
        -1
    }
}
