//! # Leetcode 3355. Zero Array Transformation I
//! https://leetcode.com/problems/zero-array-transformation-i/
//! - `Medium`; `y2025m05d19`; `Hinted`; `16ms`; `9.9mb`; `1 attempt`;
//! Topics: difference_array.

pub fn is_zero_array(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> bool {
    let len = nums.len();
    let mut diff_box_arr: Box<[i32]> = vec![0; len + 1].into_boxed_slice();

    for query in queries {
        let begin_i = query[0] as usize;
        let end_i = query[1] as usize;

        diff_box_arr[begin_i] += 1;
        diff_box_arr[end_i] -= 1;
    }

    let mut curr_num = 0;

    for (need_num, got_diff_num) in nums.into_iter().zip(diff_box_arr.iter().cloned()) {
        curr_num += got_diff_num;
        if need_num > curr_num {
            return false;
        }
    }
    true
}
