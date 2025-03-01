//! # Leetcode 873. Length of Longest Fibonacci Subsequence
//! https://leetcode.com/problems/length-of-longest-fibonacci-subsequence/
//! - `Medium`; `y2025m02d27`; `Independently Solved`; `122ms`; `2.4mb`; `3 attempts`;
//!
//! Topics: recursion/subsequence.

pub fn len_longest_fib_subseq(arr: Vec<i32>) -> i32 {
    let len = arr.len();

    fn find(
        target_num: i32,
        arr_ref: &[i32],
        mut begin_i: usize,
        mut end_i: usize,
    ) -> Option<usize> {
        while begin_i < end_i {
            let mid_i = (begin_i + end_i) / 2;
            let mid_v = arr_ref[mid_i];
            if mid_v == target_num {
                return Some(mid_i);
            } else if mid_v < target_num {
                begin_i = mid_i + 1;
            } else {
                end_i = mid_i;
            }
        }

        None
    }

    fn search_helper(
        num_0: i32,
        num_1: i32,
        curr_len: i32,
        begin_i: usize,
        arr_ref: &[i32],
        ans_max_len_mut_ref: &mut i32,
    ) {
        let target_num = num_0 + num_1;

        if let Some(next_i) = find(target_num, arr_ref, begin_i, arr_ref.len()) {
            *ans_max_len_mut_ref = (*ans_max_len_mut_ref).max(curr_len + 1);
            search_helper(
                num_1,
                target_num,
                curr_len + 1,
                next_i + 1,
                arr_ref,
                ans_max_len_mut_ref,
            );
        }
    }

    let mut ans_max_len = 0; // Second bug. Was 2 before. hahaha.

    for i in 0..len {
        for j in (i + 1)..len {
            search_helper(arr[i], arr[j], 2, j + 1, &arr, &mut ans_max_len);
        }
    }

    ans_max_len
}

/// # First Attempt
///
/// I was misunderstanding the question haha.
pub fn len_longest_fib_subseq_0(arr: Vec<i32>) -> i32 {
    let len = arr.len();
    let mut len_dp_vec: Vec<i32> = vec![1; len];
    let mut arr_iter = arr.iter().cloned().enumerate();
    arr_iter.next();
    arr_iter.next();

    fn find_dp(
        target_num: i32,
        arr_ref: &[i32],
        mut begin_i: usize,
        mut end_i: usize,
        dp_vec_mut_ref: &[i32],
    ) -> Option<i32> {
        while begin_i < end_i {
            let mid_i = (begin_i + end_i) / 2;
            let mid_v = arr_ref[mid_i];
            if mid_v == target_num {
                return Some(dp_vec_mut_ref[mid_i]);
            } else if mid_v < target_num {
                begin_i = mid_i + 1;
            } else {
                end_i = mid_i;
            }
        }

        None
    }

    let mut ans_max_len = 1;

    for (i, num_2) in arr_iter {
        for j in 1..i {
            let num_1 = arr[j];
            let target_num = num_2 - num_1;

            if let Some(prev_len) = find_dp(target_num, &arr, 0, j, &len_dp_vec) {
                len_dp_vec[i] = len_dp_vec[i].max(prev_len + 2);
                ans_max_len = ans_max_len.max(len_dp_vec[i]);
            }
        }
    }

    ans_max_len
}
