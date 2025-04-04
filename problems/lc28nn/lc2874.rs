//! # Leetcode 2874. Maximum Value of an Ordered Triplet II
//! https://leetcode.com/problems/maximum-value-of-an-ordered-triplet-ii/
//! - `Medium`; `y2025m04d03`; `Hinted`; `0ms`; `3.7mb`; `2 attempts`;
//! Topics: prefix_info_aggregation.
//!
//! Since the previous problem, `maximum-value-of-an-ordered-triplet-i`, only needs an `O(n^3)` solution, I thought an `O(n^2)` was fast enough haha.
//! I didn't think of iterating with index `j` haha.
//!

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

    let mut num_iter = nums.into_iter().take(len - 1).enumerate();

    let mut prev_max_num = num_iter.next().expect("len > 0").1;

    for (j, num) in num_iter {
        ans_num =
            ans_num.max((prev_max_num as i64 - num as i64) * max_value_info_vec[j + 1] as i64);
        prev_max_num = prev_max_num.max(num);
    }

    ans_num
}
