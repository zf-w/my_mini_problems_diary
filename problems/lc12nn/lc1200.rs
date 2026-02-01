//! # Leetcode 1200. Minimum Absolute Difference
//! https://leetcode.com/problems/minimum-absolute-difference/
//! y2025m01d26; Independently Solved

pub fn minimum_abs_difference(mut arr: Vec<i32>) -> Vec<Vec<i32>> {
    arr.sort_unstable();

    let elem0_iter = arr.iter().cloned();
    let mut elem1_iter = arr.iter().cloned();

    elem1_iter.next();

    let mut diff_min = i32::MAX;
    let mut min_diff_count = 0;

    for (e0, e1) in elem0_iter.zip(elem1_iter) {
        let diff = e1 - e0;

        if diff < diff_min {
            diff_min = diff;
            min_diff_count = 0;
        } else if diff == diff_min {
            min_diff_count += 1;
        }
    }

    let mut ans_pair_vec_vec: Vec<Vec<i32>> = Vec::with_capacity(min_diff_count);

    let elem0_iter = arr.iter().cloned();
    let mut elem1_iter = arr.iter().cloned();

    elem1_iter.next();

    for (e0, e1) in elem0_iter.zip(elem1_iter) {
        let diff = e1 - e0;

        if diff == diff_min {
            ans_pair_vec_vec.push(vec![e0, e1]);
        }
    }

    ans_pair_vec_vec
}