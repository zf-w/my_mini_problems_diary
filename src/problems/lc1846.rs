//! ## Leetcode 1846. Maximum Element Ater Decreasing and Rearranging
//! https://leetcode.com/problems/maximum-element-after-decreasing-and-rearranging
//! - `Medium`; `Independently Solved`; `2023-11-14`;
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