//! ## Leetcode 713. Subarray Product Less Than K
//! https://leetcode.com/problems/subarray-product-less-than-k
//! - `Medium`; `Independently Solved`; `2024-03-27`;
//!
//! A problem about the sliding window.

pub fn num_subarray_product_less_than_k(nums: Vec<i32>, k: i32) -> i32 {
    let mut ans: i32 = 0;
    let mut begin_i: usize = 0;
    let mut end_i: usize = 0;
    let mut curr_prod: i32 = 1;
    let len = nums.len();
    while end_i < len {
        if curr_prod * nums[end_i] < k {
            curr_prod *= nums[end_i];
            end_i += 1;
            ans += (end_i - begin_i) as i32;
            // println!("{}", ans);
        } else {
            if begin_i == end_i {
                end_i += 1;
                begin_i += 1;
            } else {
                curr_prod /= nums[begin_i];
                begin_i += 1;
            }
        }
    }
    ans
}
