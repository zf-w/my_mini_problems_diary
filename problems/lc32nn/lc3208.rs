//! # Leetcode 3208. Alternating Groups II
//! https://leetcode.com/problems/alternating-groups-ii/
//! - `Medium`; `y2025m03d09`; `Independently Solved`; `11ms`; `3.1mb`; `2 attempt`;
//! Topics: sliding_window.

pub fn number_of_alternating_groups(colors: Vec<i32>, k: i32) -> i32 {
    let k_minus_one = k as usize - 1;
    let mut color_iter = colors
        .iter()
        .cloned()
        .chain(colors.iter().cloned().take(k_minus_one));
    let mut prev_color = color_iter.next().expect("len > 0");
    let mut curr_alternating_len: usize = 1;
    let mut ans_len = 0;
    for color in color_iter {
        if color != prev_color {
            prev_color = color;
            curr_alternating_len += 1;
        } else {
            if curr_alternating_len > k_minus_one {
                ans_len += curr_alternating_len - k_minus_one;
            }
            curr_alternating_len = 1;
        }
    }
    if curr_alternating_len > k_minus_one {
        // Missed this part.
        ans_len += curr_alternating_len - k_minus_one;
    }
    ans_len as i32
}
