//! ## Leetcode 145. Binary Tree Postorder Traversal
//! https://leetcode.com/problems/binary-tree-postorder-traversal
//! - `Easy`; `Independently Solved`; `2023-12-07`;
//! 
//! The interesting part would be thinking about how to solve this using an iterative approach. The stack represents the state of the iterator. When initializing or adding a new subtree into the stack, the heads need to traverse as deep as possible by trying both left and right subtrees.
//! 
//! This problem would also be a good practice for implementing iterators in Rust.


// Definition for a binary tree node.
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
    } else if let Some(next) = node_rc.borrow().right.clone() {
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
    if self.s.is_empty() {
      return Some(top.borrow().val);
    }
    let prev_top = self.s.last().unwrap().clone();
    let prev_top_left = &prev_top.borrow().left;
    if prev_top_left.is_some() && top == *prev_top_left.as_ref().unwrap() {
      add_node_helper(prev_top.borrow().right.clone(), &mut self.s);
    }
    Some(top.clone().borrow().val)
  }
}

pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let i = TreeNodeIter::new(root);

    i.collect()
}

// /**
//  * Definition for a binary tree node.
//  * struct TreeNode {
//  *     int val;
//  *     TreeNode *left;
//  *     TreeNode *right;
//  *     TreeNode() : val(0), left(nullptr), right(nullptr) {}
//  *     TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
//  *     TreeNode(int x, TreeNode *left, TreeNode *right) : val(x), left(left), right(right) {}
//  * };
//  */

// void add_node_helper(TreeNode* root, stack<TreeNode*>& s) {
//      while (root != nullptr) {
//         s.push(root);
//         if (root->left != nullptr) {
//             root = root->left;
//         } else if (root->right != nullptr) {
//             root = root->right;
//         } else {
//             root = nullptr;
//         }
//     }
// }

// class Iter {
//     stack<TreeNode*> s;
//     public:
//     Iter(TreeNode* root) {
//         add_node_helper(root, s);
//     }
//     int* next() {
//         if (s.empty()) {
//             return nullptr;
//         }
//         TreeNode* top = s.top();
//         s.pop();
//         if (s.empty()) {
//             return new int(top->val);
//         }
//         TreeNode* prev_top = s.top();
//         if (prev_top->left == top) {
//             add_node_helper(prev_top->right, s);
//         }
//         return new int(top->val);
//     }
// };

// class Solution {
// public:
//     vector<int> postorderTraversal(TreeNode* root) {
//         Iter i(root);
//         int* v = i.next();
//         vector<int> ans;
//         while (v != nullptr) {
//             ans.push_back(*v);
//             v = i.next();
//         }
//         return ans;
//     }
// };
