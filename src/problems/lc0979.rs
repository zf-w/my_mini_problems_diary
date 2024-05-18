//! ## Leetcode 979. Distribute Coins in Binary Tree
//! https://leetcode.com/problems/distribute-coins-in-binary-tree
//! - `Medium`; `Independently Solved`; `2024-05-17`;
//!
//! I guess the core idea would be that a node can first owe some debts to pay later. This idea turns the problem into a recursion-related problem.

use std::{cell::RefCell, rc::Rc};

use crate::helpers::lc::TreeNode;

fn helper(root_rc: Rc<RefCell<TreeNode>>) -> (i32, i32) {
    let root_ref = root_rc.borrow();
    let mut root_count = 0;
    let mut root_info = root_ref.val;
    if let Some(left_rc) = root_ref.left.clone() {
        let (left_info, count) = helper(left_rc);
        root_info += left_info;
        root_count += count + (left_info - 1).abs();
    }
    if let Some(right_rc) = root_ref.right.clone() {
        let (right_info, count) = helper(right_rc);
        root_info += right_info;
        root_count += count + (right_info - 1).abs();
    }
    (root_info - 1, root_count)
}

pub fn distribute_coins(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    if let Some(root_rc) = root {
        helper(root_rc).1
    } else {
        0
    }
}
