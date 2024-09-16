//! ## Leetcode 1310. XOR Queries of a Subarray
//! https://leetcode.com/problems/xor-queries-of-a-subarray/
//! - `Medium`; `Independently Solved`; `2024-09-13`;
//!
//! This problem is about the associative and self-inverse properties of XOR.

pub fn xor_queries(arr: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
    let arr_len = arr.len();
    let mut prefix_sum_vec: Vec<i32> = Vec::with_capacity(arr_len);
    let mut curr_sum: i32 = 0;
    for arr_elem in arr {
        curr_sum ^= arr_elem;
        prefix_sum_vec.push(curr_sum);
    }
    let get_xor_value = |begin_i: usize, end_i: usize| -> i32 {
        let begin_xor_val = if begin_i == 0 {
            0
        } else {
            prefix_sum_vec[begin_i - 1]
        };
        let end_xor_val = prefix_sum_vec[end_i];
        end_xor_val ^ begin_xor_val
    };
    let mut ans_vec = Vec::with_capacity(queries.len());
    for query in queries {
        ans_vec.push(get_xor_value(query[0] as usize, query[1] as usize));
    }
    ans_vec
}
