//! # Leetcode 2918. Minimum Equal Sum of Two Arrays After Replacing Zeros
//! https://leetcode.com/problems/minimum-equal-sum-of-two-arrays-after-replacing-zeros/
//! - `Medium`; `y2025m05d10`; `Independently Solved`; `19ms`; `3.6mb`; `1 attempt`;
//! Topics: uncategorized.

pub fn min_sum(nums1: Vec<i32>, nums2: Vec<i32>) -> i64 {
    let mut num_1_has_zero_flag: bool = false;
    let mut num_2_has_zero_flag: bool = false;

    let mut num_1_min: i64 = 0;
    let mut num_2_min: i64 = 0;

    fn helper(num_arr_ref: &[i32], has_zero_mut_ref: &mut bool, min_sum_mut_ref: &mut i64) {
        for num in num_arr_ref {
            *min_sum_mut_ref += if *num == 0 {
                *has_zero_mut_ref = true;
                1
            } else {
                (*num) as i64
            };
        }
    }

    helper(&nums1, &mut num_1_has_zero_flag, &mut num_1_min);
    helper(&nums2, &mut num_2_has_zero_flag, &mut num_2_min);

    if num_1_min == num_2_min {
        num_1_min
    } else if (num_1_min < num_2_min) && num_1_has_zero_flag == true {
        num_2_min
    } else if (num_2_min < num_1_min) && num_2_has_zero_flag == true {
        num_1_min
    } else {
        -1
    }
}
