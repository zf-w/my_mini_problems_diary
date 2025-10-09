//! # Leetcode 3025. Find the Number of Ways to Place People I
//! https://leetcode.com/problems/find-the-number-of-ways-to-place-people-i/
//! - `Medium`; `y2025m09d02`; `Independently Solved`; `10ms`; `2.2mb`; `1 attempt`;
//! Topics: uncategorized.

pub fn number_of_pairs(points: Vec<Vec<i32>>) -> i32 {
    let mut ans_count = 0;
    for (i_0, point_0) in points.iter().enumerate() {
        let x_0 = point_0[0];
        let y_0 = point_0[1];
        for (i_1, point_1) in points.iter().enumerate() {
            if i_1 == i_0 {
                continue;
            }

            let x_1 = point_1[0];
            let y_1 = point_1[1];
            if x_1 > x_0 || y_1 < y_0 {
                continue;
            }

            let mut found_flag = true;

            for (i_2, point_2) in points.iter().enumerate() {
                if i_2 == i_1 || i_2 == i_0 {
                    continue;
                }

                let x_2 = point_2[0];
                let y_2 = point_2[1];

                if x_2 >= x_1 && x_2 <= x_0 && y_2 >= y_0 && y_2 <= y_1 {
                    found_flag = false;
                    break;
                }
            }

            if found_flag == true {
                ans_count += 1;
            }
        }
    }
    ans_count
}
