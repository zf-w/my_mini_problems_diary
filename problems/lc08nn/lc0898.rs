//! # Leetcode 898. Bitwise ORs of Subarrays
//! https://leetcode.com/problems/bitwise-ors-of-subarrays/
//! - `Medium`; `y2025m07d31`; `Learned from Solution`; `133ms`; `7.4mb`; `1 attempt`;
//! Topics: subarray.

pub fn subarray_bitwise_o_rs(arr: Vec<i32>) -> i32 {
    use std::collections::HashSet;

    let mut ans_hashset: HashSet<i32> = HashSet::new();

    let mut curr_hashset: HashSet<i32> = HashSet::with_capacity(32);
    curr_hashset.insert(0);

    for num in arr {
        let mut next_hashset: HashSet<i32> = HashSet::with_capacity(32);
        for curr in curr_hashset {
            next_hashset.insert(curr | num);
            ans_hashset.insert(curr | num);
        }

        next_hashset.insert(num);
        ans_hashset.insert(num);

        curr_hashset = next_hashset;
    }

    ans_hashset.len() as i32
}