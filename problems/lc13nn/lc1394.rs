//! # Leetcode 1394. Find Lucky Integer in an Array
//! https://leetcode.com/problems/find-lucky-integer-in-an-array/
//! - `Easy`; `y2025m07d05`; `Independently Solved`; `0ms`; `2.2mb`; `1 attempt`;
//! Topics: count_tracking.

pub fn find_lucky(arr: Vec<i32>) -> i32 {
    const NUM_UPPER_EX_BOUND: usize = 500;
    let mut num_count_arr: [usize; NUM_UPPER_EX_BOUND] = [0; NUM_UPPER_EX_BOUND];

    for num in arr {
        num_count_arr[num as usize - 1] += 1;
    }

    for (i, num_count) in num_count_arr.into_iter().enumerate().rev() {
        if i + 1 == num_count {
            return (i + 1) as i32;
        }
    }
    -1
}
