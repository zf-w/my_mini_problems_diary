//! # Leetcode 3228. Maximum Number of Operations to Move Ones to the End
//! https://leetcode.com/problems/maximum-number-of-operations-to-move-ones-to-the-end/
//! - `Medium`; `y2025m11d12`; `Independently Solved`; `0ms`; `2.5mb`; `1 attempt`;
//! Topics: uncategorized.

pub fn max_operations(s: String) -> i32 {
    s.chars()
        .map(|c| -> bool { c == '1' })
        .fold(
            (false, 0, 0),
            |(prev_one_flag, group_one_num, ans_count), is_one_flag| -> (bool, i32, i32) {
                match (is_one_flag, prev_one_flag) {
                    (true, _) => (true, group_one_num + 1, ans_count),
                    (false, true) => (false, group_one_num, ans_count + group_one_num),
                    (false, false) => (false, group_one_num, ans_count),
                }
            },
        )
        .2
}