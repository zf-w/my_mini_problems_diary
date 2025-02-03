//! ## Leetcode 2441. Largest Positive Integer That Exists With Its Negative
//! https://leetcode.com/problems/largest-positive-integer-that-exists-with-its-negative
//! - `Easy`; `Independently Solved`; `2024-05-02`;
//!
//! We can use an array to mark whether both positive and negative versions of a number exist in the array.

pub fn find_max_k(nums: Vec<i32>) -> i32 {
    let mut count: [(bool, bool); 1000] = [(false, false); 1000];
    for num in nums {
        let num_neg = num < 0;
        let num_i = num.abs() as usize - 1;
        if num_neg {
            count[num_i].1 = true;
        } else {
            count[num_i].0 = true;
        }
    }
    for off_i in 0..1000usize {
        let i = 999 - off_i;
        let curr_cell = &count[i];
        if curr_cell.0 == true && curr_cell.1 == true {
            return i as i32 + 1;
        }
    }
    -1
}
