//! # Leetcode 2873. Maximum Value of an Ordered Triplet I
//! https://leetcode.com/problems/maximum-value-of-an-ordered-triplet-i/
//! - `Easy`; `y2025m04d01`; `Independently Solved`; `0ms`; `2.2mb`; `1 attempt`;
//! Topics: uncategorized.

pub fn maximum_triplet_value(nums: Vec<i32>) -> i64 {
    let len = nums.len();
    let mut max_value_info_vec: Vec<i32> = vec![0; len];

    let mut max_num = nums.last().expect("len > 0").clone();
    for (num, max_value_entry_mut_ref) in nums
        .iter()
        .rev()
        .cloned()
        .zip(max_value_info_vec.iter_mut().rev())
    {
        max_num = max_num.max(num);
        *max_value_entry_mut_ref = max_num;
    }

    let mut ans_num: i64 = 0;

    for i in 0..(len - 2) {
        for j in (i + 1)..(len - 1) {
            ans_num =
                ans_num.max((nums[i] as i64 - nums[j] as i64) * max_value_info_vec[j + 1] as i64);
        }
    }
    ans_num
}
