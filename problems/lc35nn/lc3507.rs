//! # Leetcode 3507. Minimum Pair removal to Sort Array I
//! https://leetcode.com/problems/minimum-pair-removal-to-sort-array-i/
//! y2026m01d22; Hinted

pub fn minimum_pair_removal(mut nums: Vec<i32>) -> i32 {
        let mut ans_merge_count = 0;
    let mut non_decreasing_flag = false;

    while non_decreasing_flag == false {
        let len = nums.len();
        let mut num_iter = nums.iter().cloned();

        let mut prev_num = num_iter.next().expect("len > 0");

        let mut merge_sum = i32::MAX;
        let mut merge_idx: usize = 0;

        non_decreasing_flag = true;

        let mut idx = 0;
        for curr_num in num_iter {
            if curr_num < prev_num {
                non_decreasing_flag = false;
            }

            let curr_sum = curr_num + prev_num;

            if curr_sum < merge_sum {
                merge_sum = curr_sum;
                merge_idx = idx;
            }

            idx += 1;
            prev_num = curr_num;
        }

        if non_decreasing_flag == true {
            break;
        }

        ans_merge_count += 1;

        nums[merge_idx] = merge_sum;

        for i in (merge_idx + 1)..(len - 1) {
            nums[i] = nums[i + 1];
        }

        nums.resize(len - 1, 0);
    }

    ans_merge_count
}