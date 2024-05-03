//! ## Leetcode 165. Compare Version Numbers
//! https://leetcode.com/problems/compare-version-numbers
//! - `Medium`; `Independently Solved`; `2024-05-03`;
//!
//! We can parse the character string between each dot into numbers and then compare them. One interesting part would be how to handle early termination of version strings.

pub fn compare_version(version1: String, version2: String) -> i32 {
    let mut iter1 = version1.chars();
    let mut iter2 = version2.chars();

    fn get_curr_version(iter: &mut std::str::Chars) -> Option<i32> {
        let base_0 = '0' as u8;
        let mut ans = 0;
        let mut end = true;
        while let Some(c) = iter.next() {
            if c == '.' {
                break;
            }
            ans = ans * 10 + (c as u8 - base_0) as i32;
            end = false;
        }
        if end {
            None
        } else {
            Some(ans)
        }
    }
    loop {
        let curr_1 = get_curr_version(&mut iter1);
        let curr_2 = get_curr_version(&mut iter2);
        match (curr_1, curr_2) {
            (Some(v_1), Some(v_2)) => {
                if v_1 != v_2 {
                    if v_1 > v_2 {
                        return 1;
                    } else {
                        return -1;
                    }
                }
            }
            (Some(v_1), None) => {
                if v_1 > 0 {
                    return 1;
                }
            }
            (None, Some(v_2)) => {
                if v_2 > 0 {
                    return -1;
                }
            }
            (None, None) => {
                break;
            }
        }
    }
    0
}
