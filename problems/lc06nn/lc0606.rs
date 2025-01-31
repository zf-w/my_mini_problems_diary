//! ## Leetcode 606. Construct String from Binary Tree
//! https://leetcode.com/problems/construct-string-from-binary-tree
//! - `Easy`; `Independently Solved`; `2023-12-07`;
//! 
//! This would be a good question practicing the `match` feature in Rust. I guess this would be possible to solve using iterative approaches. But I feel the recursive way would be more human-friendly.

// Definition for a binary tree node.
use crate::helpers::lc::TreeNode;
use std::rc::Rc;
use std::cell::RefCell;

pub fn tree2str(root: Option<Rc<RefCell<TreeNode>>>) -> String {
  if root.is_none() {
    return String::from("");
  }
  let root_rc = root.unwrap();
  let left = root_rc.borrow().left.clone();
  let right = root_rc.borrow().right.clone();
  let v = root_rc.borrow().val;
  match (left.clone(), right.clone()) {
    (Some(_), Some(_)) => {
      format!("{}({})({})", v, tree2str(left), tree2str(right))
    },
    (Some(_), None) => {
      format!("{}({})", v, tree2str(left))
    },
    (None, Some(_)) => {
      format!("{}()({})", v, tree2str(right))
    },
    (None, None) => {
      format!("{}", v)
    }
  }
}
