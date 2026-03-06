//! # Leetcode 1784. Check if Binary String Has at Most One Segment of Ones
//! https://leetcode.com/problems/check-if-binary-string-has-at-most-one-segment-of-ones/
//! - y2026m03d06; Independently Solved;

pub fn check_ones_segment(s: String) -> bool {
    s.as_bytes()
        .iter()
        .cloned()
        .fold(
            (true, false, false, false),
            |(first_one_group_flag, prev_is_one_flag, determined_flag, ans_flag),
             c_u8|
             -> (bool, bool, bool, bool) {
                if determined_flag {
                    (true, true, true, ans_flag)
                } else {
                    if c_u8 == b'1' {
                        if first_one_group_flag {
                            (true, true, false, true)
                        } else {
                            (false, true, true, false)
                        }
                    } else {
                        (false, false, false, ans_flag)
                    }
                }
            },
        )
        .3
}