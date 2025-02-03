//! ## Leetcode 3133. Minimum Array End
//! https://leetcode.com/problems/minimum-array-end/
//! - `Medium`; `Independently Solved`; `y2024m11d08`;

pub fn min_end(n: i32, mut x: i32) -> i64 {
    let n_i64 = n as i64 - 1;
    let x_i64 = x as i64;
    let mut zero_idxs: [usize; 32] = [0; 32];
    let mut zero_len: usize = 0;
    let mut zero_i: usize = 0;
    let mut big_shift_num_i64: i64 = 1;
    while x > 0 {
        if x & 1 == 0 {
            zero_idxs[zero_len] = zero_i;
            big_shift_num_i64 += 1 << zero_i;
            zero_len += 1;
            // println!("{}", zero_i);
        }
        // println!("x: {}", x);
        zero_i += 1;
        x >>= 1;
    }
    let to_big_shift_len_i64: i64 = 1 << zero_len;
    // println!("{:b}, {}, {}, {}", x_i64, to_big_shift_len_i64, (n_i64 / to_big_shift_len_i64 ) * (x_i64 + big_shift_num_i64), big_shift_num_i64);
    let mut ans_i64: i64 = x_i64 + (n_i64 / to_big_shift_len_i64) * (x_i64 + big_shift_num_i64);
    let mut to_add_i64 = n_i64 % to_big_shift_len_i64;
    zero_i = 0;
    while to_add_i64 > 0 {
        if to_add_i64 & 1 == 1 {
            ans_i64 += 1 << zero_idxs[zero_i];
            // println!("{}, {}", zero_i, zero_idxs[zero_i]);
        }
        to_add_i64 >>= 1;
        zero_i += 1;
    }
    ans_i64
}
