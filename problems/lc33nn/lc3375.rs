//! # Leetcode 3375. Minimum Operations to Make Array Values Equal to K
//! https://leetcode.com/problems/minimum-operations-to-make-array-values-equal-to-k/
//! - `Easy`; `y2025m04d09`; `Independently Solved`; `0ms`; `2.3mb`; `1 attempt`;
//! Topics: uncategorized.

pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
    let mut num_flag_arr: [bool; 100] = [false; 100];

    let mut ans = 0;
    for num in nums {
        if num < k {
            return -1;
        }
        if num == k {
            continue;
        }
        let num_idx = num as usize - 1;
        if num_flag_arr[num_idx] == false {
            ans += 1;
        }

        num_flag_arr[num_idx] = true
    }
    ans
}
