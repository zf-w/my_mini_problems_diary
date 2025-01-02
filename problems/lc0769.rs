//! # Leetcode 769. Max Chunks To Make Sorted
//! https://leetcode.com/problems/max-chunks-to-make-sorted/
//! - `Medium`; `y2024m12d19`; `Learned from Solution`; `0ms`; `2.2mb`; `1 attempt`;
//! Learned from https://leetcode.com/problems/max-chunks-to-make-sorted/solutions/6144279/max-chunks-to-make-sorted

pub fn max_chunks_to_sorted(arr: Vec<i32>) -> i32 {
    let mut ans_count: i32 = 0;
    let mut prefix_sum = 0;
    let mut sorted_prefix_sum = 0;
    for (i, arr_elem) in arr.into_iter().enumerate() {
        prefix_sum += arr_elem;
        sorted_prefix_sum += i as i32;
        if prefix_sum == sorted_prefix_sum {
            ans_count += 1;
        }
    }
    ans_count
}
