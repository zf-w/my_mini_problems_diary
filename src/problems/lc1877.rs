//! ## Leetcode 1877. Minimize Maximum Pair Sum in Array
//! 
//! The **pair sum** of a pair `(a, b)` is equal to `a + b`.
//! The `maximum pair sum` is the largest `pair sum` in a list
//! of pairs.
//! 
//! - For example, if we have pairs `(1, 5`, `2, 3`, and `(4, 4)`, the **maximum pair sum** would be `max(1+5, 2+3, 4+4) = max(6, 5, 8) = 8`.
//! Given an array `nums` of **even** length `n`, pair up the elements of nums into `n / 2` pairs such that:
//! 
//! - Each element of `nums` is in **exactly one** pair, and
//! - The **maximum pair sum** is minimized.
//! 
//! *Return the minimized **maximum pair sum** after optimally pairing up the elements.*
//! ### Examples:
//! 
//! ```
//! use learn_cs::problems::lc1877;
//! assert_eq!(lc1877::min_pair_sum(vec![3, 5, 2, 3]), 7);
//! assert_eq!(lc1877::min_pair_sum(vec![3, 5, 4, 2, 4, 6]), 8);
//! ```
//! 
//! ### Thoughts:
//! - Hinted; 2023-11-16;
//! 
//! My intuition is to sort the array, pair numbers up from the beginning and the end, respectively, and track the maximum sum.
//! 
//! From the solution, I noticed that, since the range of the numbers is pretty limited, counting the occurrences of each number might be a quicker way.
//! 

pub fn min_pair_sum(mut nums: Vec<i32>) -> i32 {
    nums.sort();
    let mut ans: i32 = 0;
    let len: usize = nums.len();
    for i in 0..(len / 2) {
        ans = ans.max(nums[i] + nums[len - 1 - i]);
    }
    ans
}