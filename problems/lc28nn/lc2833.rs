//! # LeetCode 2833. Furthest Point From Origin
//! https://leetcode.com/problems/furthest-point-from-origin/
//! - y2026m04d23; Independently Solved;

pub fn furthest_distance_from_origin(moves: String) -> i32 {
    let (all_left_pos, all_right_pos) =
        moves
            .chars()
            .fold((0, 0), |(all_left_pos, all_right_pos), c| -> (i32, i32) {
                match c {
                    'L' => (all_left_pos - 1, all_right_pos - 1),
                    'R' => (all_left_pos + 1, all_right_pos + 1),
                    _ => (all_left_pos - 1, all_right_pos + 1),
                }
            });

    all_right_pos.max(-all_left_pos)
}