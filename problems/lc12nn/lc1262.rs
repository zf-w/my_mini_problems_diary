//! # Leetcode 1262. Greatest Sum Divisible by Three
//! https://leetcode.com/problems/greatest-sum-divisible-by-three/
//! - `Medium`; `y2025m11d23`; `Independently Solved`; `0ms`; `2.4mb`; `1 attempt`;
//! Topics: dynamic_programming.

pub fn max_sum_div_three(nums: Vec<i32>) -> i32 {
    let mut r0: i32 = 0;
    let mut r1: i32 = i32::MIN;
    let mut r2: i32 = i32::MIN;

    let mut r0_1: i32;
    let mut r1_1: i32;
    let mut r2_1: i32;

    for num in nums {
        let curr_remainder = num % 3;

        r0_1 = r0;
        r1_1 = r1;
        r2_1 = r2;

        if curr_remainder == 0 {
            r0_1 = r0_1.max(r0 + num);
            r1_1 = r1_1.max(r1 + num);
            r2_1 = r2_1.max(r2 + num);
        } else if curr_remainder == 1 {
            r0_1 = r0_1.max(r2 + num);
            r1_1 = r1_1.max(r0 + num);
            r2_1 = r2_1.max(r1 + num);
        } else {
            r0_1 = r0_1.max(r1 + num);
            r1_1 = r1_1.max(r2 + num);
            r2_1 = r2_1.max(r0 + num);
        }

        r0 = r0_1;
        r1 = r1_1;
        r2 = r2_1;
    }

    r0
}