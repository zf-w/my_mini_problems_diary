//! # Leetcode 796. Rotate String
//! https://leetcode.com/problems/rotate-string/
//! - `Easy`; `y2024m11d03`; `Independently Solved`; `0ms`; `2.1mb`; `1 attempt`;

pub fn rotate_string(s: String, goal: String) -> bool {
    let s_bytes_ref = s.as_bytes();
    let goal_bytes_ref = goal.as_bytes();
    let s_len = s.len();
    if s_len != goal_bytes_ref.len() {
        return false;
    }

    for s_begin_i in 0..s_len {
        let mut goal_i: usize = 0;
        let mut s_i = s_begin_i;
        if s_bytes_ref[s_i] != goal_bytes_ref[goal_i] {
            continue;
        }
        s_i = (s_i + 1) % s_len;
        goal_i += 1;
        while s_i != s_begin_i && s_bytes_ref[s_i] == goal_bytes_ref[goal_i] {
            s_i = (s_i + 1) % s_len;
            goal_i += 1;
        }
        if s_i == s_begin_i {
            return true;
        }
    }

    false
}
