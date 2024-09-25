//! ## Leetcode 78. Subsets
//! https://leetcode.com/problems/subsets
//! - `Medium`; `Independently Solved`; `2024-05-20`;
//!
//! When a new element arrives, we can iteratively generate the new subsets by generating all subsets with that element: all previous subsets with it appending to their ends.

pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let len = nums.len();
    let mut ans_vec: Vec<Vec<i32>> = Vec::with_capacity(2usize.pow(len as u32) as usize);
    ans_vec.push(Vec::new());
    for num in nums {
        let curr_len = ans_vec.len();
        for prev_i in 0..curr_len {
            let mut prev = ans_vec[prev_i].clone();
            prev.push(num);
            ans_vec.push(prev);
        }
    }
    ans_vec
}
