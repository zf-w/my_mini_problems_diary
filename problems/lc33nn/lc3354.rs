//! # Leetcode 3354. Make Array Elements Equal to Zero
//! https://leetcode.com/problems/make-array-elements-equal-to-zero/
//! - `Easy`; `y2025m10d28`; `Independently Solved`; `13ms`; `2.2mb`; `1 atempt`;
//! Topics: uncategorized.

pub fn count_valid_selections(nums: Vec<i32>) -> i32 {
    fn check(num_vec_ref: &[i32], mut begin_idx: usize, mut dir_left_flag: bool) -> bool {
        let mut num_box_arr: Box<[i32]> = num_vec_ref.clone().into_boxed_slice();
        let len = num_box_arr.len();
        while begin_idx < len {
            if num_box_arr[begin_idx] != 0 {
                num_box_arr[begin_idx] -= 1;
                dir_left_flag = !dir_left_flag;
            }
            if dir_left_flag {
                begin_idx -= 1;
            } else {
                begin_idx += 1;
            }
        }

        num_box_arr.iter().sum::<i32>() == 0
    }

    let mut prev_zero_flag = false;
    let mut prev_ok_num = 0;

    let len = nums.len();

    let mut ans_num = 0;

    for i in 0..len {
        if nums[i] != 0 {
            prev_zero_flag = false;
            continue;
        }

        if prev_zero_flag == true {
            ans_num += prev_ok_num;
            continue;
        }

        prev_ok_num = 0;

        if check(&nums, i, true) {
            prev_ok_num += 1;
        }

        if check(&nums, i, false) {
            prev_ok_num += 1;
        }

        ans_num += prev_ok_num;
        prev_zero_flag = true;
    }

    ans_num
}