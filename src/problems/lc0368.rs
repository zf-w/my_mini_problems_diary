//! ## Leetcode 368. Largest Divisible Subset
//! https://leetcode.com/problems/largest-divisible-subset
//! - `Medium`; `Busy Learn`; `2024-02-09`;
//!
//! An interesting problem. The structure below is pretty similar to the Union Find structure or the structure used in the UIUC CS225 SP2023 MP maze.

pub fn largest_divisible_subset(mut nums: Vec<i32>) -> Vec<i32> {
    nums.sort();
    let len = nums.len();
    let mut info: Vec<(usize, usize)> = Vec::with_capacity(len);
    info.push((len, 1)); // first bug here
    let mut best_i: usize = 0;
    let mut best_size: usize = 1;
    for i in 1..len {
        let curr = &nums[i];
        let mut prev_i = len;
        let mut curr_size: usize = 1; // second bug here, needs to be 1 instead of 0
        for back in 1..=i {
            let j = i - back;
            // if j < curr_size - 1 {
            //     break;
            // }
            let prev_val = &nums[j];
            let prev_info = info[j];
            if curr % prev_val == 0 && prev_info.1 + 1 > curr_size {
                curr_size = prev_info.1 + 1;
                prev_i = j;
            }
        }
        // println!("{i} {curr_size}");
        if curr_size > best_size {
            best_size = curr_size;
            best_i = i;
        }
        info.push((prev_i, curr_size));
    }
    let mut ans = Vec::with_capacity(best_size);
    let mut curr_i = best_i;
    while curr_i < len {
        ans.push(nums[curr_i]);
        curr_i = info[curr_i].0;
    }
    ans
}
