//! # Leetcode 11. Container With Most Water
//! https://leetcode.com/problems/container-with-most-water/
//! - `Medium`; `y2025m10d04`; `Hinted`; `0ms`; `3.2mb`; `1 attempt`;
//! Topics: two_pointers.

pub fn max_area(height: Vec<i32>) -> i32 {
    let mut l_idx: usize = 0;
    let mut r_idx: usize = height.len() - 1;
    let mut ans_max_area = 0;

    while l_idx < r_idx {
        let l_height = height[l_idx];
        let r_height = height[r_idx];

        ans_max_area = ans_max_area.max(l_height.min(r_height) * (r_idx - l_idx) as i32);

        if l_height < r_height {
            l_idx += 1;
        } else {
            r_idx -= 1;
        }
    }

    ans_max_area
}
