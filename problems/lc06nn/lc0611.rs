//! # Leetcode 611. Valid Triangle Number
//! https://leetcode.com/problems/valid-triangle-number/
//! - `Medium`; `y2025m09d26`; `Independently Solved`; `54ms`; `2.2mb`; `3 attempts`;
//! Topics: binary_search.

pub fn triangle_number(mut nums: Vec<i32>) -> i32 {
    let len = nums.len();

    if len < 3 {
        return 0;
    }

    for entry_mut_ref in nums.iter_mut() {
        *entry_mut_ref <<= 1;
    }

    nums.sort_unstable();

    let mut ans_count = 0;

    for idx0 in 0..(len - 2) {
        let num0 = nums[idx0];
        for idx1 in (idx0 + 1)..(len - 1) {
            let num1 = nums[idx1];

            let upper_ex_bound = num0 + num1 - 1;

            let search_arr_ref = &nums[(idx1 + 1)..];

            let upper_idx = idx1
                + 1
                + search_arr_ref
                    .binary_search(&upper_ex_bound)
                    .expect_err("This elem should not exist.");
            // println!("{}", upper_idx);
            let add_num = upper_idx - idx1 - 1;

            ans_count += add_num;
        }
    }

    ans_count as i32
}
