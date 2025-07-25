//! # Leetcode 3487. Maximum Unique Subarray Sum After Deletion
//! https://leetcode.com/problems/maximum-unique-subarray-sum-after-deletion/
//! - `Easy`; `y2025m07d25`; `Independently Solved`; `0ms`; `2.1mb`; `2 attempts`;
//! Topics: subarray.


pub fn max_sum(nums: Vec<i32>) -> i32 {
    use std::collections::HashSet;
    let mut num_set: HashSet<i32> = HashSet::new();

    let mut iter = nums.into_iter();

    let mut max_num = iter.next().expect("len > 0");
    let mut ans_sum = 0;
    let mut flag = false;

    if num_set.insert(max_num) && max_num > 0 {
        ans_sum += max_num;
        flag = true;
    }

    for num in iter {
        max_num = max_num.max(num);
        if num_set.insert(num) && num > 0 {
            ans_sum += num;
            flag = true
        }
    }

    if flag {
        ans_sum
    } else {
        max_num
    }
    }