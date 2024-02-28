//! ## Leetcode 513. Find Bottom Left Tree Value
//! https://leetcode.com/problems/find-bottom-left-tree-value
//! - `Medium`; `Independently Solved`; `2024-02-27`;
//!
//! A good recursion practice. The key would be thinking about what information do we need to combine the results from the subtrees.

use std::cell::RefCell;
use std::rc::Rc;

use crate::helpers::lc::TreeNode;

fn bottom_left_with_height(root: Option<Rc<RefCell<TreeNode>>>) -> (i32, i32) {
    let root_rc = if let Some(rc) = root {
        rc
    } else {
        return (0, -1);
    };
    let (l_n, l_h) = bottom_left_with_height(root_rc.borrow().left.clone());
    let (r_n, r_h) = bottom_left_with_height(root_rc.borrow().right.clone());
    if l_h < 0 && r_h < 0 {
        (root_rc.borrow().val, 0)
    } else {
        if l_h >= r_h {
            (l_n, l_h + 1)
        } else {
            (r_n, r_h + 1)
        }
    }
}

pub fn find_bottom_left_value(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    bottom_left_with_height(root).0
}
