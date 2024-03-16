//! ## Leetcode 525. Contiguous Array
//! https://leetcode.com/problems/contiguous-array
//! - `Medium`; `Learned from Solution`; `2024-03-15`;
//!
//! I have been thinking about whether performing check-and-insert for Maps is slow since we need to query an entry twice. Now, through Leetcode's Code Samples with different runtimes, I learned that I can use `Entry` to address that issue.

use std::collections::HashMap;

pub fn find_max_length(nums: Vec<i32>) -> i32 {
    let len = nums.len();
    let mut mp: HashMap<i32, usize> = HashMap::with_capacity(len * 2);
    let mut curr: i32 = 0;
    let mut ans: i32 = 0;
    for (curr_i, num) in nums.iter().enumerate() {
        if num == &0 {
            curr -= 1;
        } else {
            curr += 1;
        }
        use std::collections::hash_map::Entry::*;
        match mp.entry(curr) {
            Occupied(e) => {
                let prev_i = e.get();
                if (curr_i - prev_i) as i32 > ans {
                    ans = (curr_i - prev_i) as i32;
                }
            }
            Vacant(e) => {
                if curr == 0 {
                    if curr_i as i32 + 1 > ans {
                        ans = curr_i as i32 + 1;
                    }
                } else {
                    e.insert(curr_i);
                }
            }
        }
    }
    ans
}
