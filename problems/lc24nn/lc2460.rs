//! # Leetcode 2460. Apply Operations to an Array
//! https://leetcode.com/problems/apply-operations-to-an-array/
//! - `Easy`; `y2025m03d01`; `Independently Solved`; `0ms`; `2.4mb`; `1 attempt`;
//! Topics: uncategorized.

pub fn apply_operations(mut nums: Vec<i32>) -> Vec<i32> {
    let len = nums.len();
    let mut prev_num = nums[0];
    for i in 1..len {
        if nums[i] != prev_num {
            prev_num = nums[i];
            continue;
        }

        nums[i - 1] *= 2;
        nums[i] = 0;
        prev_num = 0
    }

    let mut head_entry_idx = 0;
    for i in 0..len {
        if nums[i] == 0 {
            continue;
        }
        nums.swap(head_entry_idx, i);
        head_entry_idx += 1;
    }

    nums
}
