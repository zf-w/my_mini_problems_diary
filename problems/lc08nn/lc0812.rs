//! # Leetcode 812. Largest Triangle Area
//! https://leetcode.com/problems/largest-triangle-area/
//! - `Easy`; `y2025m09d27`; `Independently Solved`; `1ms`; `2.3mb`; `1 attempt`;
//! Topics: uncategorized.

pub fn largest_triangle_area(points: Vec<Vec<i32>>) -> f64 {
    let point_num = points.len();

    fn calc_edge_square(p0: &[i32], p1: &[i32]) -> i32 {
        let x0 = p0[0];
        let y0 = p0[1];
        let x1 = p1[0];
        let y1 = p1[1];

        let x_diff = x1 - x0;
        let y_diff = y1 - y0;

        x_diff * x_diff + y_diff * y_diff
    }

    let mut ans_max = 0;

    for i0 in 0..(point_num - 2) {
        let p0 = &points[i0];
        for i1 in (i0 + 1)..(point_num - 1) {
            let p1 = &points[i1];

            let e0 = calc_edge_square(p0, p1);

            for i2 in (i1 + 1)..point_num {
                let p2 = &points[i2];

                let e1 = calc_edge_square(p0, p2);
                let e2 = calc_edge_square(p1, p2);

                let temp = e0 + e1 - e2;

                let pre_area = 4 * e0 * e1 - temp * temp;

                ans_max = ans_max.max(pre_area);
            }
        }
    }

    (ans_max as f64).sqrt() / 4.0
}
