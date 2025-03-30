//! # Leetcode 763. Partition Labels
//! https://leetcode.com/problems/partition-labels/
//! - `Medium`; `y2025m03d30`, `Independently Solved`; `0ms`; `2.1mb`; `1 attempt`;
//! Topics: intervals.

pub fn partition_labels(s: String) -> Vec<i32> {
    const CHAR_LOWER_A_U8: u8 = b'a';
    let s_len = s.len();
    let mut char_info_arr: [(usize, usize); 26] = [(s_len, s_len); 26];

    fn char_to_idx_map(c_u8: u8) -> usize {
        (c_u8 - CHAR_LOWER_A_U8) as usize
    }

    for (i, c_u8) in s.as_bytes().iter().cloned().enumerate() {
        let c_idx = char_to_idx_map(c_u8);
        let (char_info_begin_mut_ref, char_info_end_mut_ref) = &mut char_info_arr[c_idx];
        if *char_info_begin_mut_ref == s_len {
            *char_info_begin_mut_ref = i;
            *char_info_end_mut_ref = i + 1;
        } else {
            *char_info_end_mut_ref = i + 1;
        }
    }

    char_info_arr.sort_unstable();

    // println!("{:?}", char_info_arr);

    let mut info_arr_iter = char_info_arr.into_iter();

    let (mut group_begin, mut group_end) = info_arr_iter.next().expect("len > 0");

    let mut ans_vec: Vec<i32> = Vec::with_capacity(s_len);

    for (interval_begin, interval_end) in info_arr_iter {
        if interval_begin == s_len {
            break;
        }
        // println!("{} {}", interval_begin, interval_end);
        if interval_begin >= group_begin && interval_begin < group_end {
            group_end = group_end.max(interval_end);
        } else {
            ans_vec.push((group_end - group_begin) as i32);
            group_begin = interval_begin;
            group_end = interval_end;
        }
    }

    ans_vec.push((group_end - group_begin) as i32);

    ans_vec
}
