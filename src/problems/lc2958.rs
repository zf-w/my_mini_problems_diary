//! ## Leetcode 2958. Length of Longest Subarray With at Mosk K Frequency
//! https://leetcode.com/problems/length-of-longest-subarray-with-at-most-k-frequency
//! - `Medium`; `Independently Solved`; `2024-03-27`;
//!
//! This is a continuation of yesterday's problem, sliding window problem, I guess. The interesting part would be how to access elements within a HashMap. Rust pushes you to consider when the earliest possible time is for a mutable reference to be dropped, how mutable references can increase efficiency, and the cost in terms of safety.

use std::{collections::HashMap, ptr};

pub fn max_subarray_length(nums: Vec<i32>, k: i32) -> i32 {
    let mut ans: i32 = 0;
    let mut mp: HashMap<i32, usize> = HashMap::with_capacity(100);
    let mut begin_i: usize = 0;
    let mut end_i: usize = 0;
    let len = nums.len();
    let k_usize = k as usize;
    while end_i < len {
        let curr = nums[end_i];
        // println!("{} {}", begin_i, end_i);

        let curr_count = ptr::addr_of_mut!(*mp.entry(curr).or_insert(0));
        if unsafe { *curr_count } == k_usize {
            while begin_i < end_i {
                let to_dec = nums[begin_i];
                // println!("Dec {}", to_dec);
                begin_i += 1;
                if to_dec == curr {
                    break;
                } else {
                    *mp.get_mut(&to_dec).expect("Should have") -= 1;
                }
            }
        } else {
            // println!("Inc {}", &curr);
            unsafe { *curr_count += 1 };
        }

        end_i += 1;
        // println!("Final: {} {}", begin_i, end_i);
        ans = ans.max((end_i - begin_i) as i32);
    }
    ans
}
