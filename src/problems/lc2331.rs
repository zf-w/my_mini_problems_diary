//! ### Leetcode 2331. Evaluate Boolean Binary Tree
//! https://leetcode.com/problems/evaluate-boolean-binary-tree
//! - `Easy`; `Independently Solved`; `2024-05-16`;
//!
//! We can use recursion to recursively evaluate the subtrees and get the final answer.

use std::{cell::RefCell, rc::Rc};

use crate::helpers::lc::TreeNode;

pub fn evaluate_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    let root_rc = root.expect("Should all work due to problem constraints.");
    let root_ref = root_rc.borrow();
    match root_ref.val {
        0 => false,
        1 => true,
        2 => evaluate_tree(root_ref.left.clone()) || evaluate_tree(root_ref.right.clone()),
        3 => evaluate_tree(root_ref.left.clone()) && evaluate_tree(root_ref.right.clone()),
        _ => panic!("Invalid node value"),
    }
}
