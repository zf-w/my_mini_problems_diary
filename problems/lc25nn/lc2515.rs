//! # LeetCode 2515. Shortest Distance to Target String in a Circular Array
//! https://leetcode.com/problems/shortest-distance-to-target-string-in-a-circular-array/
//! - y2025m04d15; Independently Solved;

pub fn closest_target(words: Vec<String>, target: String, start_index: i32) -> i32 {
    let len = words.len();
    let start_idx = start_index as usize;

    if let Some(idx) = words.into_iter().enumerate().fold(
        None,
        |ans_len_opt, (target_idx, word_ref)| -> Option<usize> {
            if *word_ref != target {
                return ans_len_opt;
            }

            let (start_idx, target_idx) = if start_idx < target_idx {
                (start_idx, target_idx)
            } else {
                (target_idx, start_idx)
            };

            let candidate_len = (target_idx - start_idx).min(start_idx + len - target_idx);

            if let Some(ans_len) = ans_len_opt {
                Some(ans_len.min(candidate_len))
            } else {
                Some(candidate_len)
            }
        },
    ) {
        idx as i32
    } else {
        -1
    }
}
