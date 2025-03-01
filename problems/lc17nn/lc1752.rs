//! # Leetcode 1752. Check if Array Is Sorted and Rotated
//! https://leetcode.com/problems/check-if-array-is-sorted-and-rotated/
//! - `Easy`; `y2025m02d02`; `Medium`; `Independently Solved`; `0ms`; `2.2mb`; `2 attempts`;

pub fn check(nums: Vec<i32>) -> bool {
    let mut sort_break_point_found_flag = false;
    let mut num_iter = nums.into_iter();
    let first_num = num_iter.next().expect("len > 1");
    let mut prev_num = first_num;
    for curr_num in num_iter {
        if prev_num > curr_num {
            if sort_break_point_found_flag == true {
                return false;
            }
            sort_break_point_found_flag = true;
        }
        prev_num = curr_num;
    }

    sort_break_point_found_flag == false || prev_num <= first_num // (I forgot the "="" sign in the first attempt.)
}
