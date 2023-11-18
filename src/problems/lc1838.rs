//! ## Leetcode 1838. Frequency of the Most Frequent Element
//! https://leetcode.com/problems/frequency-of-the-most-frequent-element
//! 
//! The **frequency** of an element is the number of times it occurs in an array.
//! 
//! You are given an integer array `nums` and an integer `k`. In one operation, 
//! you can choose an index of `nums` and increment the element at that index by
//! `1`.
//! 
//! Return the `maximum possible frequency` of an element after performing at most
//! `k` operations.
//! 
//! ```
//! use learn_cs::problems::lc1838;
//! assert_eq!(3, lc1838::max_frequency(vec![4, 2, 1], 5));
//! assert_eq!(2, lc1838::max_frequency(vec![1, 4, 8, 13], 5));
//! assert_eq!(1, lc1838::max_frequency(vec![3, 9, 6], 2));
//! ```
//! 
//! ### Thoughts:
//! - Independently Solved; 2023-11-17;
//! 
//! I think this question is pretty similar to a question in STAT107 at UIUC that 
//! asked you how to maximize the median of a given array of numbers. B

pub fn max_frequency(mut nums: Vec<i32>, k: i32) -> i32 {
    let len: usize = nums.len();
    nums.sort();
    let mut i: usize = 0;
    let mut j: usize = 1;
    let mut cost: i32 = 0;
    let mut ans: i32 = 1;
    while j < len {
        while j < len && cost <= k {
            cost += ((j - i) as i32) * (nums[j] - nums[j - 1]);
            j += 1;
        }
        ans = if j == len && cost <= k {
            ans.max((j - i) as i32)
        } else {
            ans.max((j - i - 1) as i32)
        };
        cost -= nums[j - 1] - nums[i];
        i += 1;
    }
    ans
}