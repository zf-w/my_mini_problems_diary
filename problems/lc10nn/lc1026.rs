//! ## Leetcode 1026. Maximum Difference Between Node And Ancestor
//! https://leetcode.com/problems/maximum-difference-between-node-and-ancestor
//! - `Medium`; `Independently Solved`; `2024-01-10`;
//! 
//! The only information you need to calculate the maximum difference of the current node is the maximum and minimum values of its descendants.

use std::cell::RefCell;
use std::rc::Rc;

use crate::helpers::lc::TreeNode;

fn helper(root_rc: Rc<RefCell<TreeNode>>, ans: &mut i32) -> (i32, i32) {
    let root_val = root_rc.borrow().val;
    let mut min = root_val;
    let mut max = root_val;
    if let Some(left_rc) = root_rc.borrow().left.clone() {
        let (min1, max1) = helper(left_rc, ans);
        min = min.min(min1);
        max = max.max(max1);
    }
    if let Some(right_rc) = root_rc.borrow().right.clone() {
        let (min1, max1) = helper(right_rc, ans);
        min = min.min(min1);
        max = max.max(max1);
    }

    *ans = (*ans).max((root_val - min).max(max - root_val));

    (min, max)
}

pub fn max_ancestor_diff(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut ans = 0;
    if let Some(root_rc) = root {
        helper(root_rc, &mut ans);
    }
    ans
}
