//! ## Leetcode 543. Diameter of Binary Tree
//! https://leetcode.com/problems/diameter-of-binary-tree
//! - `Easy`; `Independently Solved`; `2024-02-26`;
//!
//! An interesting binary tree problem. Rust did give a lot of thought to how to avoid memory issues and undefined behaviors.

use std::cell::RefCell;
use std::rc::Rc;

use crate::helpers::lc::TreeNode;

fn helper(root: Option<Rc<RefCell<TreeNode>>>) -> (i32, i32) {
    let root_rc = if let Some(rc) = root {
        rc
    } else {
        return (0, -1);
    };

    let (l_ans, l_h) = helper(root_rc.borrow().left.clone());
    let (r_ans, r_h) = helper(root_rc.borrow().right.clone());
    (l_ans.max(r_ans).max(2 + l_h + r_h), 1 + l_h.max(r_h))
}

pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    helper(root).0
}
