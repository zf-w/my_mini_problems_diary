//! ## Leetcode 3011. Find if Array Can Be Sorted
//! https://leetcode.com/problems/find-if-array-can-be-sorted/
//! - `Medium`; `y2024m11d06`; `Independently Solved`;`0ms`; `2.2mb`; `1 attempt`;
//!
//! I was going to view a solution since I was busy and exhausted. With another glance, I suddenly realized whether the array can be sorted depends on the "connecting" elements.

pub fn can_sort_array(nums: Vec<i32>) -> bool {
    let mut num_iter = nums.iter().cloned();
    let curr_num = num_iter.next().expect("len >= 1");

    let mut prev_bit_count = curr_num.count_ones();
    let mut prev_min = i32::MIN;
    let mut curr_min: i32 = curr_num;
    let mut curr_max: i32 = curr_num;
    for num in num_iter {
        let curr_bit_count = num.count_ones();
        if curr_bit_count == prev_bit_count {
            curr_min = curr_min.min(num);
            curr_max = curr_max.max(num);
            if curr_min < prev_min {
                return false;
            }
        } else {
            if curr_min < prev_min || num < curr_max {
                return false;
            }
            prev_min = curr_max;
            prev_bit_count = curr_bit_count;
            curr_min = num;
            curr_max = num;
        }
    }
    true
}
