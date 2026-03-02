//! # Leetcode 3379. Transformed Array
//! https://leetcode.com/problems/transformed-array/
//! y2026m02d05; Independently Solved;

pub fn construct_transformed_array(nums: Vec<i32>) -> Vec<i32> {
    let len = nums.len();
    let mut ans_vec = vec![0; len];

    for (i, ans_entry_mut_ref) in ans_vec.iter_mut().enumerate() {
        let num = nums[i];

        (*ans_entry_mut_ref) = if num > 0 {
            nums[(i + (num as usize)) % len]
        } else if num < 0 {
            nums[(i + len - ((-num) as usize) % len) % len]
        } else {
            0
        };
    }

    ans_vec
}