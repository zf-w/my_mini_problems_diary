//! # Leetcode 3397. Maximum Number of Distinct Elements After Operations
//! https://leetcode.com/problems/maximum-number-of-distinct-elements-after-operations/
//! - `Medium`; `y2025m10d18`; `Independently Solved`; `14ms`; `3.4mb`; `2 attempts`;
//! Topics: sorting.

pub fn max_distinct_elements(mut nums: Vec<i32>, k: i32) -> i32 {
    nums.sort_unstable();

    let mut num_iter = nums.iter_mut();

    let first_num_entry_mut_ref = num_iter.next().expect("len > 0");
    *first_num_entry_mut_ref -= k;
    let mut prev_num = *first_num_entry_mut_ref;

    let mut ans_count = 1;

    for entry_mut_ref in num_iter {
        let curr_num = *entry_mut_ref;
        if prev_num < (curr_num + k) {
            *entry_mut_ref = (curr_num - k).max(prev_num + 1);
            ans_count += 1;
            // print!("{} ", prev_num);
            prev_num = *entry_mut_ref;
        }
    }

    ans_count
}