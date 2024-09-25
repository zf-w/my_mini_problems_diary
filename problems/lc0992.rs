//! ## Leetcode 992. Subarrays with K Different Integers
//! https://leetcode.com/problems/subarrays-with-k-different-integers
//! - `Hard`; `Learned from Solution`; `2024-03-30`;
//!
//! Intuition is indeed the key. I thought about sliding windows, but I didn't think of changing the problem into Subarrays with at most K different integers. That's brilliant.

fn subarrays_with_at_most_k_distinct(nums: &Vec<i32>, k: i32) -> i32 {
    let len = nums.len();
    let mut counts: Vec<usize> = vec![0; len + 1];
    let mut begin_i = 0;
    let mut end_i = 0;
    let mut unique_num = 0;

    let mut ans = 0;
    while end_i < len {
        let entry = &mut counts[nums[end_i] as usize];
        if entry == &0 {
            unique_num += 1;
        }
        *entry += 1;

        while begin_i <= end_i && unique_num > k {
            let entry = &mut counts[nums[begin_i] as usize];
            *entry -= 1;
            if entry == &0 {
                unique_num -= 1;
            }
            begin_i += 1;
        }

        ans += end_i as i32 - begin_i as i32;

        end_i += 1;
        // println!("{} {}", begin_i, end_i);
    }
    ans as i32
}

pub fn subarrays_with_k_distinct(nums: Vec<i32>, k: i32) -> i32 {
    let at_most_k = subarrays_with_at_most_k_distinct(&nums, k);
    let at_most_k_1 = subarrays_with_at_most_k_distinct(&nums, k - 1);
    at_most_k - at_most_k_1
}

#[test]
fn check_case_0() {
    let nums = vec![1, 2];
    let k = 1;
    assert_eq!(2, subarrays_with_k_distinct(nums, k));
}

#[test]
fn check_case_1() {
    let nums = vec![2, 1, 1, 1, 2];
    let k = 1;
    assert_eq!(8, subarrays_with_k_distinct(nums, k));
}

#[test]
fn check_case_2() {
    let nums = vec![2, 1, 2, 1, 1];
    let k = 3;
    assert_eq!(0, subarrays_with_k_distinct(nums, k));
}

#[test]
fn check_case_3() {
    let nums = vec![2, 2, 1, 2, 2, 2, 1, 1];
    let k = 2;
    assert_eq!(23, subarrays_with_k_distinct(nums, k));
}
