//! ## Leetcode 2044. Count Number of Maximum Bitwise-OR Subsets
//! https://leetcode.com/problems/count-number-of-maximum-bitwise-or-subsets/
//! - `Medium`; `Independently Solved`; `y2024m10d18`;

fn count_max_or_subsets_helper(
    nums_ref: &[i32],
    idx: usize,
    prev_num: i32,
    target_num: i32,
) -> usize {
    if prev_num & target_num == target_num {
        return 2 << (nums_ref.len() - idx - 1);
    }

    let curr_elem = nums_ref[idx];
    if idx + 1 < nums_ref.len() {
        count_max_or_subsets_helper(nums_ref, idx + 1, prev_num | curr_elem, target_num)
            + count_max_or_subsets_helper(nums_ref, idx + 1, prev_num, target_num)
    } else if prev_num | curr_elem == target_num {
        1
    } else {
        0
    }
}

pub fn count_max_or_subsets(nums: Vec<i32>) -> i32 {
    let target_i32 = nums
        .iter()
        .fold(0, |ans_num, elem| -> i32 { ans_num | elem.clone() });
    count_max_or_subsets_helper(&nums, 0, 0, target_i32) as i32
}
