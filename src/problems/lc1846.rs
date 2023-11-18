//! ## Leetcode 1846. Maximum Element Ater Decreasing and Rearranging
//! https://leetcode.com/problems/maximum-element-after-decreasing-and-rearranging
//! 
//! You are given an array of positive integers `arr`. Perform some operations (possibly none) on arr so that it satisfies these conditions:
//! - The value of the **first** element in arr must be `1`.
//! - The absolute difference between any 2 adjacent elements must be less than or equal to 1. In other words, abs(arr\[i\] - arr\[i - 1\]) <= 1 for each i where `1 <= i < arr.length` **(0-indexed)**. abs(x) is the absolute value of x.
//! 
//! There are 2 types of operations that you can perform any number of times:
//! - Decrease the value of any element of arr to a smaller positive integer.
//! - Rearrange the elements of arr to be in any order.
//! *Return the maximum possible value of an element in arr after performing the operations to satisfy the conditions.*
//! 
//! ### Examples:
//! ```
//! use learn_cs::problems::lc1846;
//! assert_eq!(3, lc1846::maximum_element_after_decrementing_and_rearranging(vec![1, 100, 1000]));
//! ```
//! 
//! ### Thoughts:
//! - Independently Solved; 2023-11-14;
//! 
//! Sort the array and see how high the last "element" can reach. I guess the important thing is how many equal elements are there in the array. After sorting the array, the equal elements will block the increment of "steps."

pub fn maximum_element_after_decrementing_and_rearranging(mut arr: Vec<i32>) -> i32 {
    arr.sort();
    let mut ub: i32 = 0;
    for i in arr.iter_mut() {
        if *i > ub {
            ub += 1;
        }
    }
    ub
}