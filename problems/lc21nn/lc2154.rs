//! # Leetcode 2154. Keep Multiplying Found Values by Two
//! https://leetcode.com/problems/keep-multiplying-found-values-by-two/
//! - `Easy`; `y2025m11d19`; `Independently Solved`; `0ms`; `2.1MiB`; `2 attempts`;
//! Topics: uncategorized.

pub fn find_final_value(nums: Vec<i32>, mut original: i32) -> i32 {
        const NUM_ARR_LEN: usize = 1001;
        let mut num_arr: Box<[bool]> = vec![false; NUM_ARR_LEN].into_boxed_slice();

        for num in nums {
            num_arr[num as usize] = true;
        }

        while (original as usize) < NUM_ARR_LEN && num_arr[original as usize] == true {
            original <<= 1;
        }

        original
    }