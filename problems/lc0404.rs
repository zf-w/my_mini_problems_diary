//! ## Leetcode 404. Sum of Left Leaves
//! https://leetcode.com/problems/sum-of-left-leaves
//! - `Easy`; `Independently Solved`; `2024-04-14`;
//!
//! Learning to use pointers. This problem is about representing whether a node is the left child of the root.

use std::cell::RefCell;

use std::rc::Rc;

use crate::helpers::lc::TreeNode;

fn helper(root_ptr: *const TreeNode, left: bool) -> i32 {
    let root_ref = unsafe { root_ptr.as_ref().expect("Should be a valid pointer") };
    let left_opt = root_ref.left.as_ref();
    let right_opt = root_ref.right.as_ref();
    let mut ans = 0;
    let mut is_leaf = true;
    if let Some(left_rc) = left_opt {
        is_leaf = false;
        ans += helper(left_rc.as_ptr(), true);
    }

    if let Some(right_rc) = right_opt {
        is_leaf = false;
        ans += helper(right_rc.as_ptr(), false);
    }
    if is_leaf && left {
        ans += root_ref.val;
    }
    ans
}

pub fn sum_of_left_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let root_node = if let Some(root_rc) = root {
        root_rc.as_ptr()
    } else {
        return 0;
    };
    let root_ref = unsafe { root_node.as_ref().unwrap() };
    let left_opt = root_ref.left.as_ref();
    let right_opt = root_ref.right.as_ref();
    let mut ans = 0;

    if let Some(left_rc) = left_opt {
        ans += helper(left_rc.as_ptr(), true);
    }

    if let Some(right_rc) = right_opt {
        ans += helper(right_rc.as_ptr(), false);
    }

    ans
}
