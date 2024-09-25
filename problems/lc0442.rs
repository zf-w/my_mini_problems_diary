//! ## Leetcode 442. Find All Duplicates in an Array
//! https://leetcode.com/problems/find-all-duplicates-in-an-array
//! - `Medium`; `Learned from Solution`; `2024-03-24`;
//!
//! The question asks me to use only constant extra space. I would say using the "first bit" also counts as using an `O(n)` space. In this sense, I guess much space was wasted if the number didn't take enough space. If someone has enough time and is sure that some of the provided `u64`s are only half used, maybe they can put some `i32` into the idle space.

pub fn find_duplicates(mut nums: Vec<i32>) -> Vec<i32> {
    let len = nums.len();
    let mut ans: Vec<i32> = Vec::with_capacity(len / 2);
    for i in 0..len {
        let curr = nums[i].abs();

        let curr_i = curr as usize - 1;
        if nums[curr_i] < 0 {
            ans.push(curr);
        }
        nums[curr_i] *= -1;
    }
    ans
}
