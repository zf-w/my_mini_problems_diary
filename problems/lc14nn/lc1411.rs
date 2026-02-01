//! # Leetcode 1411. Number of Ways to Paint N x 3 Grid
//! https://leetcode.com/problems/number-of-ways-to-paint-n-3-grid/
//! - `Hard`; `y2026m01d02`; `Hinted`; `6ms`; `2.1mb`; `1 attempt`;

pub fn num_of_ways(n: i32) -> i32 {
    const MODULO: i64 = 1_000_000_007;
    const PAINT_NEXT_MAP_ARR: [&[usize]; 12] = [
        &[1, 2, 4, 5, 10],
        &[0, 3, 6, 8, 11],
        &[0, 3, 6, 7],
        &[1, 2, 7, 10],
        &[0, 6, 8, 9],
        &[0, 6, 7, 9, 10],
        &[1, 2, 4, 5, 11],
        &[2, 3, 5, 11],
        &[1, 4, 9, 10],
        &[4, 5, 8, 11],
        &[0, 3, 5, 8, 11],
        &[1, 6, 7, 9, 10],
    ];

    let mut dp_arr: [i32; 24] = [1; 24];

    let modulo_add_closure =
        |a: i32, b: i32| -> i32 { (((a as i64) + (b as i64)) % MODULO) as i32 };

    let mut prev_off = 0;
    let mut curr_off = 12;

    let mut curr_sum = 12;

    for _ in 1..n {
        for curr_color_i in 0..12 {
            let mut curr_color_sum = 0;

            for prev_color_i in PAINT_NEXT_MAP_ARR[curr_color_i].iter().cloned() {
                curr_color_sum =
                    modulo_add_closure(dp_arr[prev_off + prev_color_i], curr_color_sum);
            }

            dp_arr[curr_off + curr_color_i] = curr_color_sum;

            curr_sum = modulo_add_closure(curr_sum, curr_color_sum);
        }
        std::mem::swap(&mut curr_off, &mut prev_off);
    }
    
    curr_sum
}