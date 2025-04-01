//! # Leetcode 3191. Minimum Operations to Make Binary Array Elements Equal to One I
//! https://leetcode.com/problems/minimum-operations-to-make-binary-array-elements-equal-to-one-i/
//! - `Medium`; `y2025m03d19`; `Hinted`; `3ms`; `2.6mb`; `1 attempt`;
//! Topics: uncategorized.

pub fn min_operations(mut nums: Vec<i32>) -> i32 {
    let len = nums.len();

    let mut ans_count: i32 = 0;

    fn flip(num_ref: &mut i32) {
        *num_ref = (*num_ref + 1) % 2;
    }

    for i in 3..=len {
        let curr_arr_mut_ref: &mut [i32] = &mut nums[(i - 3)..i];

        if curr_arr_mut_ref[0] == 0 {
            curr_arr_mut_ref[0] = 1;
            flip(&mut curr_arr_mut_ref[1]);
            flip(&mut curr_arr_mut_ref[2]);
            ans_count += 1;
        }
    }
    // println!("{:?}", nums);
    if nums[(len - 3)..len]
        .iter()
        .fold(true, |prev, v| -> bool { prev & (*v == 1) })
    {
        ans_count
    } else {
        -1
    }
}
