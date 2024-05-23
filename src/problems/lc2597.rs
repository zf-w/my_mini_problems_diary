//! ## Leetcode 2597. The Number of Beautiful Subsets
//! https://leetcode.com/problems/the-number-of-beautiful-subsets
//! - `Medium`; `Independently Solved`; `2024-05-23`;
//!
//! I found the length of the numbers array is relatively small, and then I added an additional loop to check if the previously-calculated arrays are beautiful for the current element, modifying the previous problem of "LC 78. Subsets" a bit. I guess there are more efficient methods, perhaps a prefix tree structure.

pub fn beautiful_subsets(nums: Vec<i32>, k: i32) -> i32 {
    let len = nums.len();
    let mut ans_vec: Vec<Vec<i32>> = Vec::with_capacity(2usize.pow(len as u32) as usize);
    ans_vec.push(Vec::new());
    for num in nums {
        let curr_len = ans_vec.len();
        for prev_i in 0..curr_len {
            let mut is_beautiful = true;
            for prev_v in ans_vec[prev_i].iter() {
                if (prev_v - num).abs() == k {
                    is_beautiful = false;
                    break;
                }
            }
            if is_beautiful {
                let mut prev = ans_vec[prev_i].clone();
                prev.push(num);
                ans_vec.push(prev);
            }
        }
    }
    ans_vec.len() as i32 - 1
}
