//! # Leetcode 2221. Find Triangular Sum of an Array
//! https://leetcode.com/problems/find-triangular-sum-of-an-array/
//! - `Medium`; `y2025m09d30`; `Independently Solved`; `11ms`; `2.1mb`; `2 attempts`;
//! Topics: dynamic_programming.

pub fn triangular_sum(mut nums: Vec<i32>) -> i32 {
    let len = nums.len();
    for level_i in (1..len).rev() {
        for i in 0..level_i {
            nums[i] = (nums[i] + nums[i + 1]) % 10;
        }
    }

    nums[0]
}

pub fn triangular_sum_for_better_m_i_guess(nums: Vec<i32>) -> i32 {
    let mut multiplier = 1;
    let len_i32 = nums.len() as i32;
    let mut i = 1;
    let mut ans_sum = 0;

    for num in nums {
        ans_sum = (ans_sum + (num * (multiplier % 10)) % 10) % 10;
        println!("{}", multiplier);
        multiplier = (multiplier * (len_i32 - i)) / i; // Needs Inverse Here.
        i += 1;
    }
    ans_sum
}
