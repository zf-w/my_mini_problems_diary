//! ## Leetcode 1630. Arithmetic Subarrays
//! https://leetcode.com/problems/arithmetic-subarrays
//! - `Medium`; `Independently Solved`; `2023-11-22`;
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
