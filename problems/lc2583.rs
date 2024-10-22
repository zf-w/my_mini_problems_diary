//! ## Leetcode 2583. Kth Largest Sum in a Binary Tree
//! https://leetcode.com/problems/kth-largest-sum-in-a-binary-tree/
//! - `Medium`; `Independently Solved`; `y2024m10d22`;

fn kth_largest_level_sum_helper(
    level_usize: usize,
    level_sums_vec_mut_ref: &mut Vec<i64>,
    sub_root: Rc<RefCell<TreeNode>>,
) {
    if level_sums_vec_mut_ref.len() == level_usize {
        level_sums_vec_mut_ref.push(0);
    }

    let sub_root_borrow = sub_root.borrow();
    level_sums_vec_mut_ref[level_usize] += sub_root_borrow.val as i64;
    let next_level_usize = level_usize + 1;
    if let Some(left_sub_root_rc) = sub_root_borrow.left.clone() {
        kth_largest_level_sum_helper(next_level_usize, level_sums_vec_mut_ref, left_sub_root_rc);
    }
    if let Some(right_sub_root_rc) = sub_root_borrow.right.clone() {
        kth_largest_level_sum_helper(next_level_usize, level_sums_vec_mut_ref, right_sub_root_rc);
    }
}

pub fn kth_largest_level_sum(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i64 {
    let root_rc = if let Some(root_rc) = root {
        root_rc
    } else {
        return -1;
    };
    let mut level_sums_vec: Vec<i64> = Vec::new();
    kth_largest_level_sum_helper(0, &mut level_sums_vec, root_rc);
    let k_usize = k as usize;

    let level_sums_len = level_sums_vec.len();
    if k_usize >= level_sums_len {
        return -1;
    }
    level_sums_vec.sort_unstable();

    level_sums_vec[level_sums_len - k_usize]
}
