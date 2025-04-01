//! # Leetcode 3394. Check if Grid can be Cut into Sections
//! https://leetcode.com/problems/check-if-grid-can-be-cut-into-sections/
//! - `Medium`; `y2025m03d25`; `Independently Solved`; `23ms`; `10.44mb`; `1 attempt`;
//! Topics: intervals.

pub fn check_valid_cuts(n: i32, rectangles: Vec<Vec<i32>>) -> bool {
    fn check_one_direction(bound_arr_ref: &[(i32, i32)]) -> bool {
        let mut bound_iter = bound_arr_ref.iter().cloned();
        let mut group_end = bound_iter.next().expect("len > 0").1;
        let mut found_flag = false;
        for (begin, end) in bound_iter {
            if begin >= group_end {
                // println!("begin: {}", begin);
                if found_flag {
                    return true;
                }
                found_flag = true;
            }
            group_end = group_end.max(end);
        }
        false
    }

    let len = rectangles.len();
    let mut hori_bound_vec: Vec<(i32, i32)> = Vec::with_capacity(len);
    let mut vert_bound_vec: Vec<(i32, i32)> = Vec::with_capacity(len);

    for rectangle_vec in rectangles {
        let begin_x = rectangle_vec[0];
        let begin_y = rectangle_vec[1];
        let end_x = rectangle_vec[2];
        let end_y = rectangle_vec[3];
        hori_bound_vec.push((begin_x, end_x));
        vert_bound_vec.push((begin_y, end_y));
    }

    hori_bound_vec.sort_unstable();
    vert_bound_vec.sort_unstable();

    check_one_direction(&hori_bound_vec) || check_one_direction(&vert_bound_vec)
}
