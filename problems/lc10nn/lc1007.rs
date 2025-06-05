//! # Leetcode 1007. Minimum Domino Rotations For Equal Row
//! https://leetcode.com/problems/minimum-domino-rotations-for-equal-row/
//! - `Medium`; `y2025m05d03`; `Independently Solved`; `0ms`; `2.3mb`; `1 attempt`;
//! Topics: uncategorized.

pub fn min_domino_rotations(tops: Vec<i32>, bottoms: Vec<i32>) -> i32 {
    let len = tops.len();
    let mut num_pair_iter = tops.into_iter().zip(bottoms);

    let mut opt_0: Option<(i32, usize, usize)> = None;
    let mut opt_1: Option<(i32, usize, usize)> = None;

    if let Some((top_num, bottom_num)) = num_pair_iter.next() {
        if top_num != bottom_num {
            opt_0 = Some((top_num, 1, 0));
            opt_1 = Some((bottom_num, 0, 1));
        } else {
            opt_0 = Some((top_num, 1, 1));
        }
    }

    for (top_num, bottom_num) in num_pair_iter {
        if opt_0.is_none() && opt_1.is_none() {
            break;
        }
        if let Some((num, top_count, bottom_count)) = opt_0 {
            opt_0 = if top_num == num && bottom_num == num {
                Some((num, top_count + 1, bottom_count + 1))
            } else if top_num == num {
                Some((num, top_count + 1, bottom_count))
            } else if bottom_num == num {
                Some((num, top_count, bottom_count + 1))
            } else {
                None
            }
        }

        if let Some((num, top_count, bottom_count)) = opt_1 {
            opt_1 = if top_num == num && bottom_num == num {
                Some((num, top_count + 1, bottom_count + 1))
            } else if top_num == num {
                Some((num, top_count + 1, bottom_count))
            } else if bottom_num == num {
                Some((num, top_count, bottom_count + 1))
            } else {
                None
            }
        }
    }

    let calc_rotation_num = |top_count: usize, bottom_count: usize| -> i32 {
        (len - top_count.max(bottom_count)) as i32
    };

    match (opt_0, opt_1) {
        (None, None) => -1,
        (None, Some((_, top_1, bottom_1))) => calc_rotation_num(top_1, bottom_1),
        (Some((_, top_0, bottom_0)), None) => calc_rotation_num(top_0, bottom_0),
        (Some((_, top_0, bottom_0)), Some((_, top_1, bottom_1))) => {
            calc_rotation_num(top_1, bottom_1).min(calc_rotation_num(top_0, bottom_0))
        }
    }
}
