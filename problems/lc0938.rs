//! ## Leetcode 938. Range Sum of BST
//! https://leetcode.com/problems/range-sum-of-bst
//! - `Esay`; `Independently Solved`; `2024-01-08`;
//! 
//! An interesting question about recursion.

use std::rc::Rc;
use std::cell::RefCell;

use crate::helpers::lc::TreeNode;

pub fn range_sum_bst(root: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> i32 {
    let root_rc: Rc<RefCell<TreeNode>>;
    if low > high {
        return 0;
    }
    if let Some(r) = root {
      root_rc = r;
    } else {
      return 0;
    }

    let curr_val = root_rc.borrow().val;
    if curr_val < low {
      range_sum_bst(root_rc.borrow().right.clone(), low, high)
    } else if curr_val >= low && curr_val <= high {
        // println!("{}", curr_val);
      curr_val + range_sum_bst(root_rc.borrow().left.clone(), low, curr_val - 1) + range_sum_bst(root_rc.borrow().right.clone(), curr_val + 1, high)
    } else {
      range_sum_bst(root_rc.borrow().left.clone(), low, high)
    }
}