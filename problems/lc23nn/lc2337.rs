//! # Move Pieces to Obtain a String
//! https://leetcode.com/problems/move-pieces-to-obtain-a-string/
//! - `Medium`; `y2024m12d05`; `Independently Solved`; `4ms`; `2.7mb`; `1 attempt`;

pub fn can_change(start: String, target: String) -> bool {
    let mut start_iter = start.char_indices();
    let mut target_iter = target.char_indices();
    fn get_next_info(char_iter_mut_ref: &mut std::str::CharIndices) -> Option<(bool, usize)> {
        for (idx, c) in char_iter_mut_ref.by_ref() {
            if c == '_' {
                continue;
            }
            return Some((c == 'L', idx));
        }
        None
    }
    loop {
        let start_opt = get_next_info(&mut start_iter);
        let target_opt = get_next_info(&mut target_iter);
        if start_opt.is_none() && target_opt.is_none() {
            break;
        }
        let ((start_is_left, start_i), (target_is_left, target_i)) =
            if let (Some(start_info), Some(target_info)) = (start_opt, target_opt) {
                (start_info, target_info)
            } else {
                return false;
            };
        if start_is_left != target_is_left
            || (start_is_left && start_i < target_i)
            || (start_is_left == false && target_i < start_i)
        {
            return false;
        }
    }
    true
}
