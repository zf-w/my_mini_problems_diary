//! ## Leetcode 94. Binary Tree Inorder Traversal
//! https://leetcode.com/problems/binary-tree-inorder-traversal
//! - `Easy`; `Independently Solved`; `2023-12-08`;
//! 
//! This problem is pretty similar to the postorder one. The intuition would be similar. You need to first think about the internal state of the iteration implementation, think about the first initial state, and finally, the transition between states utill the end.


use crate::helpers::lc::TreeNode;
use std::rc::Rc;
use std::cell::RefCell;

struct TreeNodeIter {
  s: Vec<Rc<RefCell<TreeNode>>>
}

fn add_node_helper(mut root: Option<Rc<RefCell<TreeNode>>>, s: &mut Vec<Rc<RefCell<TreeNode>>>) {
  while root.is_some() {
    let node_rc = root.unwrap();
    s.push(node_rc.clone());
    if let Some(next) = node_rc.borrow().left.clone() {
      root = Some(next);
    } else {
      root = None;
    };
  }
}

impl TreeNodeIter {
  pub fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
    let mut s: Vec<Rc<RefCell<TreeNode>>> = Vec::new();
    add_node_helper(root, &mut s);
    Self {
      s
    }
  }
}

impl Iterator for TreeNodeIter {
  type Item = i32;

  fn next(&mut self) -> Option<Self::Item> {
    if self.s.is_empty() {
      return None;
    }
    let top = self.s.pop().unwrap();
    
    add_node_helper(top.borrow().right.clone(), &mut self.s);
    
    Some(top.clone().borrow().val)
  }
}

pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let i = TreeNodeIter::new(root);

    i.collect()
}