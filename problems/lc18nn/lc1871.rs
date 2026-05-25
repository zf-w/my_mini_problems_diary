//! # LeetCode 1871. Jump Game VII
//! https://leetcode.com/problems/jump-game-vii
//! - y2026m05d25; Learned from Solution

pub fn can_reach(s: String, min_jump: i32, max_jump: i32) -> bool {
    let len = s.len();
    let min_jump = min_jump as usize;
    let max_jump = max_jump as usize;

    let mut dp_reachable_flag_arr_box = vec![false; len].into_boxed_slice();
    let mut prefix_zero_count_arr_box: Box<[usize]> = vec![1; len].into_boxed_slice();

    dp_reachable_flag_arr_box[0] = true;
    let s_u8_arr_ref = s.as_bytes();

    for idx in min_jump..len {
        let is_zero_flag = s_u8_arr_ref[idx] == b'0';

        let reachable_flag = if is_zero_flag == true {
            let right_idx = idx - min_jump;
            let zero_in_range_count = prefix_zero_count_arr_box[right_idx]
                - if max_jump >= idx {
                    0
                } else {
                    prefix_zero_count_arr_box[idx - max_jump - 1]
                };
            
            // println!("{} at {}", zero_in_range_count, idx);

            zero_in_range_count > 0
        } else {
            false
        };

        dp_reachable_flag_arr_box[idx] = reachable_flag;

        // println!("{} at {}", dp_reachable_flag_arr_box[idx], idx);

        prefix_zero_count_arr_box[idx] =
            prefix_zero_count_arr_box[idx - 1] + if reachable_flag { 1 } else { 0 };
    }

    dp_reachable_flag_arr_box[len - 1]
}