//! # Leetcode 120. Triangle
//! https://leetcode.com/problems/triangle/
//! - `Medium`; `y2025m09d25`; `Independently Solved`; `0ms`; `2.3mb`; `1 attempt`;
//! Topics: dynamic_programming.

pub fn minimum_total(mut triangle: Vec<Vec<i32>>) -> i32 {
    let mut prev_layer_vec = triangle.pop().expect("layer_num > 0");
    while let Some(mut curr_layer_vec) = triangle.pop() {
        let mut prev_1_iter = prev_layer_vec.iter().cloned();
        prev_1_iter.next();
        let prev_0_iter = prev_layer_vec.iter().cloned();
        for (entry_mut_ref, (prev_cost_0, prev_cost_1)) in
            curr_layer_vec.iter_mut().zip(prev_0_iter.zip(prev_1_iter))
        {
            *entry_mut_ref += prev_cost_0.min(prev_cost_1);
        }

        prev_layer_vec = curr_layer_vec;
    }

    prev_layer_vec[0]
}
