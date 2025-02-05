//! # Leetcode 1790. Check if One String Swap Can Make Strings Equal
//! https://leetcode.com/problems/check-if-one-string-swap-can-make-strings-equal/
//! - `Easy`; `y2025m02d04`; `Independently Solved`; `0ms`; `2.3mb`; `3 attempts`;

pub fn are_almost_equal(s1: String, s2: String) -> bool {
    let mut first_diff_opt: Option<(char, char)> = None;
    let mut diff_checked: bool = false;
    for (c_1, c_2) in s1.chars().zip(s2.chars()) {
        if c_1 == c_2 {
            continue;
        }
        if diff_checked {
            return false;
        }
        if let Some((prev_c_1, prev_c_2)) = first_diff_opt {
            if prev_c_1 != c_2 || prev_c_2 != c_1 {
                return false;
            }

            diff_checked = true;
        } else {
            first_diff_opt = Some((c_1, c_2));
        }
    }
    diff_checked || first_diff_opt.is_none() // Was `true` in the first attempt and was `first_diff_opt.is_none()` in the second attempt haha.
}
