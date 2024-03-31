//! ## Leetcode 3101. Counting Alternating Subarrays
//! https://leetcode.com/problems/count-alternating-subarrays
//! - `Medium`; `Independently Solved`; `2024-03-30`;
//!
//! Another subarray problem. All subarrays of an alternating subarray are alternating.

pub fn count_alternating_subarrays(nums: Vec<i32>) -> i64 {
    let mut ans = 0;
    let mut begin_i: usize = 0;
    let mut end_i: usize = 1;
    let len = nums.len();
    let mut prev = nums[0];
    while end_i <= len {
        while end_i < len && nums[end_i] == prev ^ 1 {
            prev = nums[end_i];
            end_i += 1;
        }
        // println!("{} {}", begin_i, end_i);
        let curr_len = (end_i - begin_i) as i64;
        ans += (curr_len * curr_len + curr_len) / 2;
        begin_i = end_i;
        end_i += 1;
        if end_i < len {
            prev = nums[begin_i];
        }
    }
    ans
}

#[test]
fn check_case_0() {
    let nums = vec![0, 1, 1, 1];
    assert_eq!(5, count_alternating_subarrays(nums));
}

#[test]
fn check_case_1() {
    let nums = vec![0, 1, 0, 1];
    assert_eq!(10, count_alternating_subarrays(nums));
}
