//! ## Leetcode 1325. Delete Leaves wWith a Given Value
//! https://leetcode.com/problems/delete-leaves-with-a-given-value
//! - `Medium`; `Independently Solved`; `2024-05-17`;
//!
//! We can recursively delete the leaf nodes with the target value from bottom to top(root).

use std::{cell::RefCell, rc::Rc};

use crate::helpers::lc::TreeNode;

pub fn remove_leaf_nodes(
    root: Option<Rc<RefCell<TreeNode>>>,
    target: i32,
) -> Option<Rc<RefCell<TreeNode>>> {
    let root_rc = if let Some(root_rc) = root {
        root_rc
    } else {
        return None;
    };
    let mut root_ref = root_rc.borrow_mut();
    root_ref.left = remove_leaf_nodes(root_ref.left.clone(), target);
    root_ref.right = remove_leaf_nodes(root_ref.right.clone(), target);
    if root_ref.left.is_none() && root_ref.right.is_none() && root_ref.val == target {
        None
    } else {
        drop(root_ref);
        Some(root_rc)
    }
}
