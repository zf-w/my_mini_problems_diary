//! ## Leetcode 1630. Arithmetic Subarrays
//! - `Medium`
//! 
//! A sequence of numbers is called **arithmetic** if consists of at least two elements, and the difference between every two consecutive elements is the same. More formally, a sequence `s` is arithmetic if and only if `s[i+1] - s[i] == s[1] - s[0]` for all valid `i`.
//! 
//! For example, there are **arithmetic** sequences:
//! 
//! > 1, 3, 5, 7, 9
//! 
//! > 7, 7, 7, 7
//! 
//! > 3, -1, -5, -9
//! 
//! You are given an array of `n` integers, `nums`, and two arrays of `m` integers each, `l` and `r`, representing the `m` range queries, where the `i^th` query is the range `[l[i], r[i]]`. All the arrays are **0-indexed**.
//! 
//! Return *a list of `boolean` elements `answer`, where `answer[i]` is `true` if the subarray `nums[l[i]..r[i]]` can be **rearranged** to form an **arithmetic** sequence, and `false` otherwise*.
//! 
//! ### Example:
//! ```
//! use learn_cs::problems::lc1630;
//! let nums = vec![4, 6, 5, 9, 3, 7];
//! let l = vec![0, 0, 2];
//! let r = vec![2, 3, 5];
//! let expected = vec![true, false, true];
//! 
//! assert_eq!(expected, lc1630::check_arithmetic_subarrays(nums, l, r));
//! ```
//! 
//! ### Thoughts:
//! - `Independently Solved`; `2023-11-22`;
//! 
//! My intuition on how to check whether a subarray is arithmetic is to calculate the expected differences of elements and see if all the elements can be found in the subarray in `O(len of subarray)`. Another way might be copying and sorting the subarray.
//! 
//! Be careful with the inclusive upper index of `r` and the special case of all elements are the same.


fn check_arithmetic_subarray(nums: &[i32]) -> bool {
    let len: usize = nums.len();
    if len <= 1 {
        return true;
    }
    let min: i32 = *nums.iter().min().unwrap();
    let max: i32 = *nums.iter().max().unwrap();
    if max == min {
        return true;
    }
    let len_i32: i32 = len as i32 - 1;
    if (max - min) % len_i32 > 0 {
        return false;
    }
    let step: i32 = (max - min) / len_i32;

    let mut found: Vec<bool> = vec![false; len];

    for num in nums.iter() {
        if ((num - min) % step) > 0 {
            return false;
        }
        let idx = ((num - min) / step) as usize;
        // println!("{}, {}", num, idx);
        if idx < len && found[idx] != true {
            found[idx] = true;
        } else {
            return false;
        }
    }
    true
}

pub fn check_arithmetic_subarrays(nums: Vec<i32>, l: Vec<i32>, r: Vec<i32>) -> Vec<bool> {
    let task_len: usize = l.len();
    let mut ans: Vec<bool> = Vec::with_capacity(task_len);
    for idx in 0..task_len {
        let i = l[idx] as usize;
        let j = r[idx] as usize + 1;
        ans.push(check_arithmetic_subarray(&nums[i..j]));
    }
    ans
}
