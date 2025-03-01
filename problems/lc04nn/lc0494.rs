//! # Leetcode 494. Target Sum
//! https://leetcode.com/problems/target-sum/
//! - `Medium`; `y2024m12d26`; `Independently Solved`; `127ms`; `2.3mb`; `1 attempt`;

fn find_target_sum_ways_helper(num_arr: &[i32], target: i32) -> i32 {
    let first_elem = num_arr.first().expect("len > 0");
    if num_arr.len() == 1 {
        let mut ans_num = 0;
        if *first_elem == target {
            ans_num += 1
        };
        if *first_elem == -target {
            ans_num += 1
        };
        return ans_num;
    }

    let next_arr = &num_arr[1..];

    find_target_sum_ways_helper(next_arr, target + first_elem)
        + find_target_sum_ways_helper(next_arr, target - first_elem)
}

pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
    find_target_sum_ways_helper(&nums, target)
}
