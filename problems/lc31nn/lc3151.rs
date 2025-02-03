//! # Leetcode 3151. Special Array I
//! https://leetcode.com/problems/special-array-i/
//! - `Easy`; `72025m02d01`; `Easy`; `Independently Solved`; `0ms`; `2.3mb`; `1 attempt`;

pub fn is_array_special(nums: Vec<i32>) -> bool {
    let mut num_iter = nums.into_iter();
    let mut prev_elem = num_iter.next().expect("len > 1");
    for num in num_iter {
        if (prev_elem & 1) == (num & 1) {
            return false;
        }
        prev_elem = num;
    }
    true
}
