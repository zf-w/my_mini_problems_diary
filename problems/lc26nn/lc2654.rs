//! # Leetcode 2654. Minimum Number of Operations to Make All Array Elements Equal to 1
//! https://leetcode.com/problems/minimum-number-of-operations-to-make-all-array-elements-equal-to-1/
//! - `Medium`; `y2025m11d12`; `Learned from Solution`; `0ms`; `2.2mb`; `4 attempts`;
//! Topics: subarray.

pub fn min_operations(nums: Vec<i32>) -> i32 {
    fn greatest_common_divisor(mut num_0: i32, mut num_1: i32) -> i32 {
        while num_1 != 0 {
            let temp = num_1;
            num_1 = num_0 % num_1;
            num_0 = temp;
        }

        num_0
    }

    let mut num1_count = 0;

    for num in nums.iter().cloned() {
        if num == 1 {
            num1_count += 1;
        }
    }

    let len = nums.len();

    if num1_count > 0 {
        return len as i32 - num1_count;
    }

    let mut min_prime_dis_opt: Option<i32> = None;

    for (idx_0, num_0) in nums.iter().cloned().enumerate().take(len - 1) {
        let mut subarr_gcd = num_0;

        if subarr_gcd == 1 {
            min_prime_dis_opt = Some(-1);
        }

        for (idx_1, num_1) in nums.iter().cloned().enumerate().skip(idx_0) {
            subarr_gcd = greatest_common_divisor(subarr_gcd, num_1);

            if subarr_gcd > 1 {
                continue;
            }

            let subarr_dis = (idx_1 - idx_0) as i32;

            if let Some(min_prime_dis) = min_prime_dis_opt.as_mut() {
                *min_prime_dis = (*min_prime_dis).min(subarr_dis);
            } else {
                min_prime_dis_opt = Some(subarr_dis);
            }

            break;
        }
    }

    let min_prime_dis = if let Some(min_prime_dis) = min_prime_dis_opt {
        min_prime_dis
    } else {
        return -1;
    };

    min_prime_dis + len as i32 - 1
}