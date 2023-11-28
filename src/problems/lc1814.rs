//! ## Leetcode 1814. Count Nice Pairs in an Array
//! - `Medium`
//! 
//! You are given an array `nums` that consists of non-negative integers. Let us define `rev(x)` as the reverse of the non-negative integer `x`. For example, `rev(120) = 21`. A pair of indices `(i, j)` is **nice** if it satisfies all of the following conditions:
//! 
//! - `0 <= i < j < nums.length`
//! - `nums[i] + rev(nums[j]) == nums[j] + rev(nums[i])`
//! 
//! Return *the number of nice pairs of indices*. Since that number can be too large, return it modulo `1e9 + 7`.
//! 
//! ### Example:
//! ```
//! use learn_cs::problems::lc1814;
//! 
//! let nums = vec![42, 11, 1, 97];
//! assert_eq!(2, lc1814::count_nice_pairs(nums));
//! ```
//! 
//! ### Thoughts:
//! - `Independently Solved`; `2023-11-20`
//! 
//! I feel that the following transformation of the definition equation would be key to solving this problem: `nums[i] - rev(nums[i]) == nums[j] - rev(nums[j])`. And then, you can easily calculate the number of pairs by grouping those numbers with equal `x - rev(x)`. How to safely calculate the inverse modulo would be the key to solving this problem.
//! 



use std::collections::hash_map::HashMap;

pub fn count_nice_pairs(nums: Vec<i32>) -> i32 {
    fn rev(mut num: i32) -> i32 {
        let mut rev_num: i32 = 0;
        while num > 0 {
            rev_num = rev_num * 10 + num % 10;
            num /= 10;
        }
        rev_num
    }
    let len: usize = nums.len();
    let mut ct: HashMap<i32, u32> = HashMap::with_capacity(len * 2);
    for num in nums.iter() {
        let k: i32 = *num - rev(*num);
        match ct.get_mut(&k) {
            Some(v) => {
                *v += 1;
            },
            None => {
                ct.insert(k, 1);
            }
        }
    }

    let mut tot: u64 = 0;
    let m: u64 = 1000000007;
    let i2: u64 = 500000004;
    for (_, vr) in ct.iter() {
        let v: u64 = *vr as u64;
        tot = (tot + (((v * (v - 1) % m) * i2) % m)) % m;
    }

    tot as i32
}