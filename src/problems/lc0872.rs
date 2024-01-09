//! ## Leetcode 872. Leaf-Similar Trees
//! https://leetcode.com/problems/leaf-similar-trees
//! - `Easy`; `Independently Solved`; `2024-01-08`;
//!
//! I guess this problem could be solved by an iterator-like algorithm, which can return false any time finding a mismatch.
//!

use std::cell::RefCell;
use std::rc::Rc;

use crate::helpers::lc::TreeNode;

fn traverse(root: Option<Rc<RefCell<TreeNode>>>, leaves: &mut Vec<i32>) {
    let root_rc: Rc<RefCell<TreeNode>>;
    if let Some(curr_rc) = root {
        root_rc = curr_rc;
    } else {
        return;
    }

    let left = root_rc.borrow().left.clone();
    let right = root_rc.borrow().right.clone();
    traverse(left.clone(), leaves);
    if left.is_none() && right.is_none() {
        leaves.push(root_rc.borrow().val);
    }
    traverse(right, leaves);
}

pub fn leaf_similar(
    root1: Option<Rc<RefCell<TreeNode>>>,
    root2: Option<Rc<RefCell<TreeNode>>>,
) -> bool {
    let mut leaves1: Vec<i32> = Vec::with_capacity(200);
    let mut leaves2: Vec<i32> = Vec::with_capacity(200);
    traverse(root1, &mut leaves1);
    traverse(root2, &mut leaves2);
    leaves1 == leaves2
}
