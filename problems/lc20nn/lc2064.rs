//! ## Leetcode 2064. Minimized Maximum of Products Distributed to Any Store
//! https://leetcode.com/problems/minimized-maximum-of-products-distributed-to-any-store/
//! - `Medium`; `y2024m11d14`; `Independently Solved`;`16ms`; `3.3mb`; `1 attempt`;

pub fn minimized_maximum(n: i32, quantities: Vec<i32>) -> i32 {
    let stores_usize = n as usize;
    let (q_sum_i32, mut end_q) = quantities.iter().fold(
        (0, 0),
        |(q_sum_i32, q_max_i32), elem_i32_ref| -> (i32, i32) {
            (q_sum_i32 + elem_i32_ref, q_max_i32.max(*elem_i32_ref))
        },
    );
    let mut begin_q = if q_sum_i32 % n == 0 {
        q_sum_i32 / n
    } else {
        (q_sum_i32 / n) + 1
    };

    while begin_q < end_q {
        let mut need_stores_usize: usize = 0;
        let mid_q = (begin_q + end_q) / 2;
        for quantity_ref in quantities.iter() {
            need_stores_usize += if quantity_ref % mid_q == 0 {
                quantity_ref / mid_q
            } else {
                (quantity_ref / mid_q) + 1
            } as usize;
        }
        if need_stores_usize > stores_usize {
            // Small bug here, be careful with the direction and whether to include equal.
            begin_q = mid_q + 1;
        } else {
            end_q = mid_q;
        }
    }
    begin_q
}
