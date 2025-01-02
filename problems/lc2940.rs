//! # Leetcode 2940. Find Building Where Alice and Bob Can Meet
//! https://leetcode.com/problems/find-building-where-alice-and-bob-can-meet/
//! - `Hard`; `y2024m12d22`; `Hinted`; `8ms`; `5.9mb`; `1 attempt`

pub fn leftmost_building_queries(heights: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
    let queries_len = queries.len();
    let mut ans_vec: Vec<i32> = vec![-1; queries_len];
    let mut query_vec: Vec<(usize, usize, usize)> = queries
        .into_iter()
        .enumerate()
        .map(|(query_i, query_vec)| -> (usize, usize, usize) {
            let a_i = query_vec[0] as usize;
            let b_i = query_vec[1] as usize;
            if a_i < b_i {
                (b_i, a_i, query_i)
            } else {
                (a_i, b_i, query_i)
            }
        })
        .collect();
    query_vec.sort_unstable();
    let mut prev_i = heights.len();
    let mut height_iter = heights.iter().cloned().enumerate().rev();

    let mut height_mono_stk: Vec<(i32, usize)> = Vec::with_capacity(heights.len());

    for (upper_i, lower_i, query_i) in query_vec.into_iter().rev() {
        for _ in 0..(prev_i - upper_i) {
            let (height_i, height) = height_iter.next().expect("Should less than length");
            while let Some((last_height, _)) = height_mono_stk.last().cloned() {
                if last_height <= height {
                    height_mono_stk.pop();
                } else {
                    break;
                }
            }

            height_mono_stk.push((height, height_i));
        }
        // print!("{:?}", height_mono_stk);
        prev_i = upper_i;
        let upper_height = heights[upper_i];
        let lower_height = heights[lower_i];
        let ans_entry_mut_ref = ans_vec
            .get_mut(query_i)
            .expect("Should have enough query entries in [ans_entry_mut_ref].");
        if lower_i == upper_i {
            *ans_entry_mut_ref = lower_i as i32;
            continue;
        }
        if lower_height < upper_height {
            *ans_entry_mut_ref = upper_i as i32;
            continue;
        }

        let mut begin_i: usize = 0;
        let mut end_i: usize = height_mono_stk.len();
        // print!(" {} ", lower_height);
        while begin_i < end_i {
            let mid_i = (begin_i + end_i) / 2;
            let mid_v_ref = height_mono_stk.get(mid_i).expect("Should within range.");
            if mid_v_ref.0 <= lower_height {
                end_i = mid_i;
            } else {
                begin_i = mid_i + 1;
            }
        }
        // println!("->{}", begin_i);
        if begin_i > 0 {
            *ans_entry_mut_ref = height_mono_stk[begin_i - 1].1 as i32;
        }
    }
    ans_vec
}
