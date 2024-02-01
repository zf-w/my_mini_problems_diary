//! ## Leetcode 2966. Divide Array Into Arrays With Max Difference
//! https://leetcode.com/problems/divide-array-into-arrays-with-max-difference
//! - `Medium`; `Independently Solved`; `2024-02-01`;
//!
//! Sorting is important when it comes to the difference between elements.

pub fn divide_array(mut nums: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
    let len = nums.len() / 3;
    let mut ans: Vec<Vec<i32>> = Vec::with_capacity(len);
    nums.sort();

    for i in 0..len {
        let i3 = i * 3;
        let mut curr: Vec<i32> = Vec::with_capacity(3);
        let (a, b, c) = (nums[i3], nums[i3 + 1], nums[i3 + 2]);
        if b - a <= k && c - b <= k && c - a <= k {
            curr.push(nums[i3]);
            curr.push(nums[i3 + 1]);
            curr.push(nums[i3 + 2]);
        } else {
            return vec![];
        }
        ans.push(curr);
    }
    ans
}
