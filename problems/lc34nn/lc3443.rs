//! # Leetcode 3443. Maximum Manhattan Distance After K Changes
//! https://leetcode.com/problems/maximum-manhattan-distance-after-k-changes/
//! - `Medium`; `y2025m06d20`; `Independently Solved`; `34ms`; `2.5mb`; `1 attempt`;
//! Topics: uncategorized.

pub fn max_distance(s: String, k: i32) -> i32 {
    let s_byte_arr_ref = s.as_bytes();

    let mut n_count: i32 = 0;
    let mut s_count: i32 = 0;
    let mut e_count: i32 = 0;
    let mut w_count: i32 = 0;

    let calc_dis = |n_count: i32, s_count: i32, e_count: i32, w_count: i32| -> i32 {
        let (y_max, y_min) = if n_count > s_count {
            (n_count, s_count)
        } else {
            (s_count, n_count)
        };

        let (x_max, x_min) = if e_count > w_count {
            (e_count, w_count)
        } else {
            (w_count, e_count)
        };

        let ans_diff = y_max - y_min + x_max - x_min;

        ans_diff
            + if k >= y_min + x_min {
                (y_min + x_min) << 1
            } else {
                k << 1
            }
    };

    let mut ans_max_diff = 0;

    for c_u8 in s_byte_arr_ref.iter().cloned() {
        match c_u8 {
            b'N' => {
                n_count += 1;
            }
            b'S' => {
                s_count += 1;
            }
            b'E' => {
                e_count += 1;
            }
            _ => {
                w_count += 1;
            }
        }

        ans_max_diff = ans_max_diff.max(calc_dis(n_count, s_count, e_count, w_count));
    }

    ans_max_diff
}
