//! # Leetcode 3453. Separate Squares I
//! https://leetcode.com/problems/separate-squares-i/
//! - `Medium`; `y2025m01d13`; `Learned from Solution`; `71ms`; `7.1mb`; `2 attempt`;

pub fn separate_squares(squares: Vec<Vec<i32>>) -> f64 {
    let square_info_vec: Vec<(i32, i32)> = squares
        .into_iter()
        .map(|v| -> (i32, i32) { (v[1], v[2]) })
        .collect();

    let mut square_info_iter = square_info_vec.iter().cloned();
    let first_square_ref = square_info_iter.next().expect("len > 0");

    let mut total_area = (first_square_ref.1 as f64) * (first_square_ref.1 as f64); // Second group of bugs here, about precisions.
    let mut upper_y = (first_square_ref.0 as f64 + first_square_ref.1 as f64);
    let mut lower_y = first_square_ref.0 as f64;

    for (y, l) in square_info_iter {
        let l_f64 = l as f64;
        total_area += l_f64 * l_f64;

        lower_y = lower_y.min(y as f64);
        upper_y = upper_y.max(y as f64 + l_f64);
    }

    let target_area = total_area / 2.0;

    // println!("{}", target_area);
    while (upper_y - lower_y).abs() > 0.0000005 { // First local testing bug here. Reversing the comparision operator.
        let mid_y = (upper_y + lower_y) / 2.0;

        let mut below_area = 0.0;

        for (y, l) in square_info_vec.iter().cloned() {
            if (y as f64) > mid_y {
                continue;
            }

            let l_f64 = l as f64;

            let y_length = if (y as f64 + l_f64) < mid_y {
                l_f64
            } else {
                mid_y - (y as f64)
            };

            below_area += y_length * l_f64;
        }

        // println!("{} {}", mid_y, below_area);

        if below_area >= target_area {
            upper_y = mid_y;
        } else {
            lower_y = mid_y + f64::MIN_POSITIVE;
        }
    }

    lower_y
}