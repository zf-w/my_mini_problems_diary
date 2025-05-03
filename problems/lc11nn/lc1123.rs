//! # Leetcode 1123. Lowest Common Ancestor of Deepest Leaves
//! https://leetcode.com/problems/lowest-common-ancestor-of-deepest-leaves/
//! - `Medium`; `y2025m04d03`; `Independently Solved`; `0ms`; `2.3mb`; `2 attempts`;
//! Topics: tree/binary_tree/recursion_design.

use std::{cell::RefCell, rc::Rc};

use learn_cs::mod_helpers::lc_mod::TreeNode;

pub fn lca_deepest_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    fn helper_lca_deepest_leaves(root_rc: Rc<RefCell<TreeNode>>) -> (usize, Rc<RefCell<TreeNode>>) {
        let root_rc_borrow = root_rc.borrow();
        match (root_rc_borrow.left.clone(), root_rc_borrow.right.clone()) {
            (None, None) => (0, std::rc::Rc::clone(&root_rc)),
            (None, Some(right_root_rc)) => {
                let mut right_ans = helper_lca_deepest_leaves(right_root_rc);
                right_ans.0 += 1;
                right_ans
            }
            (Some(left_root_rc), None) => {
                let mut left_ans = helper_lca_deepest_leaves(left_root_rc);
                left_ans.0 += 1;
                left_ans
            }
            (Some(left_root_rc), Some(right_root_rc)) => {
                let mut left_ans = helper_lca_deepest_leaves(left_root_rc);

                let mut right_ans = helper_lca_deepest_leaves(right_root_rc);

                if left_ans.0 == right_ans.0 {
                    (right_ans.0 + 1, std::rc::Rc::clone(&root_rc))
                } else if left_ans.0 < right_ans.0 {
                    right_ans.0 += 1; // Forgot to add one in the first attempt. Haha.
                    right_ans
                } else {
                    left_ans.0 += 1;
                    left_ans
                }
            }
        }
    }

    let root_rc = root?;

    Some(helper_lca_deepest_leaves(root_rc).1)
}
