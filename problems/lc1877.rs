//! ## Leetcode 1877. Minimize Maximum Pair Sum in Array
//! https://leetcode.com/problems/minimize-maximum-pair-sum-in-array
//! - `Medium`; `Hinted`; `2023-11-16`;
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