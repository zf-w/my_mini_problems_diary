//! # Leetcode 1161. Maximum Level Sum of a Binary Tree
//! https://leetcode.com/problems/maximum-level-sum-of-a-binary-tree/
//! - `Easy`; `y2025m01d06`; `Independently Solved`; `8ms`; `3mb`; `2 attempts`;

pub fn max_level_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut level_sum_vec: Vec<i32> = Vec::new();

    fn max_level_sum_aux(
        root_rc: Rc<RefCell<TreeNode>>,
        level_idx: usize,
        level_sum_vec_mut_ref: &mut Vec<i32>,
    ) {
        let node_borrow = root_rc.borrow();

        let node_val = node_borrow.val;

        if level_idx >= level_sum_vec_mut_ref.len() {
            level_sum_vec_mut_ref.push(node_val)
        } else {
            level_sum_vec_mut_ref[level_idx] += node_val;
        }

        if let Some(l_node_rc) = node_borrow.left.clone() {
            max_level_sum_aux(l_node_rc, level_idx + 1, level_sum_vec_mut_ref);
        }

        if let Some(r_node_rc) = node_borrow.right.clone() {
            max_level_sum_aux(r_node_rc, level_idx + 1, level_sum_vec_mut_ref);
        }
    }

    max_level_sum_aux(root.expect("node num > 0"), 0, &mut level_sum_vec);

    (level_sum_vec
        .iter()
        .cloned()
        .enumerate()
        .fold(
            (0, i32::MIN),
            |(max_idx, max_val), (idx, v)| -> (usize, i32) {
                if v > max_val {
                    (idx, v)
                } else {
                    (max_idx, max_val)
                }
            },
        )
        .0
        + 1) as i32
}