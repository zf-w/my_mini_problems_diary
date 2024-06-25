//! ## Leetcode 1038. Binary Search Tree to Greater Sum Tree
//! https://leetcode.com/problems/binary-search-tree-to-greater-sum-tree/
//! - `Medium`; `Independently Solved`; `2024-06-24`;
//!
//! We can solve this problem using a reverse pre-order traversal to calculate the sum of values greater than a node's value.

use std::cell::RefCell;
use std::rc::Rc;

use crate::helpers::lc::TreeNode;
fn bst_to_gst_helper(root: Rc<RefCell<TreeNode>>, curr_sum: &mut i32) {
    if let Some(right_rc_cell) = root.borrow().right.clone() {
        bst_to_gst_helper(right_rc_cell, curr_sum);
    }

    *curr_sum += root.borrow().val;
    root.borrow_mut().val = *curr_sum;
    if let Some(left_rc_cell) = root.borrow().left.clone() {
        bst_to_gst_helper(left_rc_cell, curr_sum);
    }
}

pub fn bst_to_gst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    let mut curr_sum = 0;
    if let Some(root_rc_ref) = root.clone() {
        bst_to_gst_helper(root_rc_ref, &mut curr_sum);
    }
    root
}
