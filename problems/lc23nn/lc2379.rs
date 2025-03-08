//! # Leetcode 2379. Minimum recolors to Get K Consecutive Black Blocks
//! https://leetcode.com/problems/minimum-recolors-to-get-k-consecutive-black-blocks/
//! - `Easy`; `y2025m03d07`; `Independently Solved`; `0ms`; `2.2mb`; `1 attempt`;
//! Topics: sliding_window.

pub fn minimum_recolors(blocks: String, k: i32) -> i32 {
    const CHAR_UPPER_W_U8: u8 = b'W';

    fn tell_if_char_is_upper_w(c_u8_ref: &u8) -> bool {
        *c_u8_ref == CHAR_UPPER_W_U8
    }

    let mut white_bool_end_iter = blocks.as_bytes().iter().map(tell_if_char_is_upper_w);
    let mut white_count: usize = 0;
    for _ in 0..k {
        let is_white_flag = white_bool_end_iter.next().expect("Should have such len...");
        if is_white_flag {
            white_count += 1;
        }
    }

    // println!("{}", white_count);

    let mut ans_white_count = white_count;

    let white_bool_begin_iter = blocks.as_bytes().iter().map(tell_if_char_is_upper_w);

    for (prev_is_white, curr_is_white) in white_bool_begin_iter.zip(white_bool_end_iter) {
        if prev_is_white {
            white_count -= 1;
        }

        if curr_is_white {
            white_count += 1;
        }

        ans_white_count = ans_white_count.min(white_count);
    }

    ans_white_count as i32
}
