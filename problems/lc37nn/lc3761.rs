//! # LeetCode 3761. Minimum Absolute Distance Between Mirror Pairs
//! https://leetcode.com/problems/minimum-absolute-distance-between-mirror-pairs/
//! - y2026m04d17; Independently Solved;

pub fn min_mirror_pair_distance(nums: Vec<i32>) -> i32 {
        use std::collections::HashMap;

        let len = nums.len();
        let mut num_to_idx_map: HashMap<i32, usize> = HashMap::with_capacity(len);

        fn reverse(mut num: i32) -> i32 {
            let mut ans_num = 0;

            while num > 0 {
                ans_num = ans_num * 10 + num % 10;
                num /= 10;
            }

            ans_num
        }

        let mut ans_diff = len;

        for (idx, num) in nums.into_iter().enumerate() {
            let num_rev = reverse(num);

            if let Some(prev_num_rev_idx) = num_to_idx_map.get(&num).cloned() {
                ans_diff = ans_diff.min(idx - prev_num_rev_idx);
            }

            *(num_to_idx_map.entry(num_rev).or_default()) = idx;
        }

        if ans_diff == len { -1 } else { ans_diff as i32 }
}