//! ## Leetcode 1814. Count Nice Pairs in an Array
//! - `Medium`; `Independently Solved`; `2023-11-20`;
//! 
//! I feel that the following transformation of the definition equation would be key to solving this problem: `nums[i] - rev(nums[i]) == nums[j] - rev(nums[j])`. And then, you can easily calculate the number of pairs by grouping those numbers with equal `x - rev(x)`. How to safely calculate the inverse modulo would be the key to solving this problem.

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