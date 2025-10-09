//! # Leetcode 1039. Minimum Score Triangulation of Polygon
//! https://leetcode.com/problems/minimum-score-triangulation-of-polygon/
//! - `Medium`; `y2025m09d29`; `Hinted`; `0ms`; `2.1mb`; `2 attempts`;
//! Topics: dynamic_programming.

pub fn min_score_triangulation(values: Vec<i32>) -> i32 {
    let len = values.len();
    let len_plus_one = len + 1;
    let mut mem_arr = vec![i32::MAX; (len_plus_one) * (len_plus_one)].into_boxed_slice();
    let index_closure = |begin_i: usize, end_i: usize| -> usize { begin_i * len_plus_one + end_i };

    fn triangulation_helper(
        begin_i: usize,
        end_i: usize,
        mem_mut_ref: &mut [i32],
        values: &[i32],
        index_closure_ref: &impl Fn(usize, usize) -> usize,
    ) -> i32 {
        if end_i - begin_i < 3 {
            return 0;
        }

        let mem_idx = index_closure_ref(begin_i, end_i);

        if mem_mut_ref[mem_idx] != i32::MAX {
            return mem_mut_ref[mem_idx];
        }

        let node_0_val = values[begin_i];
        let node_2_val = values[end_i - 1];

        let mut ans = i32::MAX;

        for node_1_idx in (begin_i + 1)..(end_i - 1) {
            let node_1_val = values[node_1_idx];
            let curr_tri_val = node_0_val * node_1_val * node_2_val;

            ans = ans.min(
                curr_tri_val
                    + triangulation_helper(
                        begin_i,
                        node_1_idx + 1,
                        mem_mut_ref,
                        values,
                        index_closure_ref,
                    )
                    + triangulation_helper(
                        node_1_idx,
                        end_i,
                        mem_mut_ref,
                        values,
                        index_closure_ref,
                    ),
            )
        }

        mem_mut_ref[mem_idx] = ans;

        ans
    };

    triangulation_helper(0, len, &mut mem_arr, &values, &index_closure)
}
