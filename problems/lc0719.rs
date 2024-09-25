//! ## Leetcode 719. Find K-th Smallest Pair Distance
//! https://leetcode.com/problems/find-k-th-smallest-pair-distance/
//! `Hard`; `Learned from Solution`; `2024-08-14`;
//!
//! Learned from https://leetcode.com/problems/find-k-th-smallest-pair-distance/solutions/5633163/2-approaches-max-heap-and-binary-search-on-answers-with-explaination/

use std::cmp::Ordering;

fn cmp(nums_arr_ref: &[i32], mid_dis: i32, k: usize) -> Ordering {
    let mut count: usize = 0;
    let mut begin_i: usize = 0;
    for end_i in 1..nums_arr_ref.len() {
        while nums_arr_ref[end_i] - nums_arr_ref[begin_i] <= mid_dis {
            // Can also apply binary search here.
            begin_i += 1;
        }
        count += end_i - begin_i;
    }
    count.cmp(&k)
}

pub fn smallest_distance_pair(mut nums: Vec<i32>, k: i32) -> i32 {
    let k: usize = k as usize;
    nums.sort_unstable();
    let mut begin_dis: i32 = 0;
    let mut end_dis: i32 = nums[0] - nums.last().expect("Getting the last elem");
    while begin_dis < end_dis {
        let mid_dis = (begin_dis + end_dis) / 2;

        match cmp(&nums, mid_dis, k) {
            Ordering::Less => {
                begin_dis = mid_dis + 1;
            }
            Ordering::Equal => return mid_dis,
            Ordering::Greater => {
                end_dis = mid_dis;
            }
        }
    }
    unreachable!()
}
