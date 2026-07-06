//! # LeetCode 1288. Removed Covered Intervals
//! https://leetcode.com/problems/remove-covered-intervals
//! - y2026m07d06; Independently Solved

pub fn remove_covered_intervals(intervals: Vec<Vec<i32>>) -> i32 {
    let mut interval_vec: Vec<(i32, i32)> = intervals
        .into_iter()
        .map(|v| -> (i32, i32) { (v[0], v[1]) })
        .collect();

    interval_vec
        .sort_unstable_by_key(|(left_num, right_num)| -> (i32, i32) { (*left_num, -(*right_num)) });

    let mut interval_iter = interval_vec.into_iter();

    let (_, mut right_bound) = interval_iter.next().expect("len > 0");

    let mut ans_count = 1;

    for (_, right_num) in interval_iter {
        if right_num <= right_bound {
            continue;
        }

        ans_count += 1;
        right_bound = right_num;
    }

    ans_count
}