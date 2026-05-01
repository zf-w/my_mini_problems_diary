//! # LeetCode 3741. Minimum Distance Between Three Equal Elements II
//! https://leetcode.com/problems/minimum-distance-between-three-equal-elements-ii
//! - y2026m04d10; Independently Solved;

pub fn minimum_distance(nums: Vec<i32>) -> i32 {
    type Info = (usize, [usize; 2]);
    let len = nums.len();
    let mut value_info_arr_box: Box<[Info]> = vec![(0, [0, 0]); len].into_boxed_slice();

    fn info_push(info_mut_ref: &mut Info, idx: usize) -> Option<usize> {
        let (len_mut_ref, idx_arr_mut_ref) = info_mut_ref;

        match *len_mut_ref {
            0 => {
                *len_mut_ref = 1;
                idx_arr_mut_ref[0] = idx;
                None
            }
            1 => {
                *len_mut_ref = 2;
                idx_arr_mut_ref[1] = idx;
                None
            }
            _ => {
                let ans = Some((idx - idx_arr_mut_ref[0]) << 1);

                idx_arr_mut_ref[0] = idx_arr_mut_ref[1];
                idx_arr_mut_ref[1] = idx;

                ans
            }
        }
    }

    let mut ans_min_dis = (len << 1) as i32;

    for (i, num) in nums.into_iter().enumerate() {
        let num_value_idx = num as usize - 1;

        if let Some(candidate_dis) = info_push(&mut value_info_arr_box[num_value_idx], i) {
            ans_min_dis = ans_min_dis.min(candidate_dis as i32);
        }
    }


    if ans_min_dis == (len << 1) as i32 {
        -1
    } else {
        ans_min_dis
    }
}