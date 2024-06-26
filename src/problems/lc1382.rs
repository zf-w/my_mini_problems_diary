//! ## Leetcode 1382. Balance a Binary Search Tree
//! https://leetcode.com/problems/balance-a-binary-search-tree/
//! - `Medium`; `Independently Solved`; `2024-06-25`;
//!
//! We can first use in-order traversal to get the sorted nodes and then reconstruct the tree with divide-and-conquer: select the root node and recursively build its left and right sub-tree.

use std::cell::RefCell;
use std::rc::Rc;

use crate::helpers::lc::TreeNode;
pub fn balance_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    let root_rc = if let Some(root_rc) = root {
        root_rc
    } else {
        return root;
    };
    let mut nodes: Vec<Rc<RefCell<TreeNode>>> = Vec::new();
    fn trav(root_rc: Rc<RefCell<TreeNode>>, collect: &mut Vec<Rc<RefCell<TreeNode>>>) {
        if let Some(left_rc) = root_rc.borrow().left.clone() {
            trav(left_rc, collect);
        }
        collect.push(root_rc.clone());
        if let Some(right_rc) = root_rc.borrow().right.clone() {
            trav(right_rc, collect);
        }
        root_rc.borrow_mut().left = None;
        root_rc.borrow_mut().right = None;
    }

    trav(root_rc, &mut nodes);

    fn build(root_rc: Rc<RefCell<TreeNode>>, go_left: bool, nodes: &[Rc<RefCell<TreeNode>>]) {
        if nodes.len() == 0 {
            return;
        }
        let mid_i = nodes.len() / 2;
        if go_left {
            let curr_node = nodes[mid_i].clone();
            root_rc.borrow_mut().left = Some(curr_node.clone());
            build(curr_node.clone(), true, &nodes[0..mid_i]);
            build(curr_node, false, &nodes[(mid_i + 1)..]);
        } else {
            let curr_node = nodes[mid_i].clone();
            root_rc.borrow_mut().right = Some(curr_node.clone());
            build(curr_node.clone(), true, &nodes[0..mid_i]);
            build(curr_node, false, &nodes[(mid_i + 1)..]);
        }
    }
    let mid_i = nodes.len() / 2;
    let root_rc = nodes[mid_i].clone();
    build(root_rc.clone(), true, &nodes[0..mid_i]);
    build(root_rc.clone(), false, &nodes[(mid_i + 1)..]);

    Some(root_rc)
}
