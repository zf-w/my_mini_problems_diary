//! ## Leetcode 2458. Height of Binary Tree After Subtree Removal Queries
//! https://leetcode.com/problems/height-of-binary-tree-after-subtree-removal-queries/
//! - `Hard`; `Independently Solved`; `y2024m10d25`;
//!
//! I guess this might be solved by some lazy forwarding techniques. Glad to see the simpler version also works.

use std::{cell::RefCell, collections::HashMap, rc::Rc};

use learn_cs::helpers::lc::TreeNode;

fn tree_queries_helper(
    subroot_rc: Rc<RefCell<TreeNode>>,
    node_info_vec_mut_ref: &mut Vec<i32>,
    val_to_idx_map_mut_ref: &mut HashMap<i32, usize>,
) -> (i32, usize, usize) {
    let (left_height, left_tree_begin_i, left_tree_end_i) =
        if let Some(left_rc_ref) = &subroot_rc.borrow().left {
            let left_rc = Rc::clone(left_rc_ref);
            tree_queries_helper(left_rc, node_info_vec_mut_ref, val_to_idx_map_mut_ref)
        } else {
            let len = node_info_vec_mut_ref.len();
            (0, len, len)
        };

    val_to_idx_map_mut_ref.insert(subroot_rc.borrow().val, node_info_vec_mut_ref.len());
    node_info_vec_mut_ref.push(-1);

    let (right_height, right_tree_begin_i, right_tree_end_i) =
        if let Some(right_rc_ref) = &subroot_rc.borrow().right {
            let right_rc = Rc::clone(right_rc_ref);
            tree_queries_helper(right_rc, node_info_vec_mut_ref, val_to_idx_map_mut_ref)
        } else {
            let len = node_info_vec_mut_ref.len();
            (0, len, len)
        };

    for node_ans_height in node_info_vec_mut_ref[left_tree_begin_i..left_tree_end_i].iter_mut() {
        (*node_ans_height) = (*node_ans_height + 1).max(right_height);
    }

    for node_ans_height in node_info_vec_mut_ref[right_tree_begin_i..right_tree_end_i].iter_mut() {
        (*node_ans_height) = (*node_ans_height + 1).max(left_height);
    }

    (
        left_height.max(right_height) + 1,
        left_tree_begin_i,
        right_tree_end_i,
    )
}

pub fn tree_queries(root: Option<Rc<RefCell<TreeNode>>>, queries: Vec<i32>) -> Vec<i32> {
    let root_rc = if let Some(root_rc) = root {
        root_rc
    } else {
        unreachable!()
    };

    let mut val_to_idx_map: HashMap<i32, usize> = HashMap::new();
    let mut node_info_vec: Vec<i32> = Vec::new();
    tree_queries_helper(root_rc, &mut node_info_vec, &mut val_to_idx_map);

    let mut ans_vec: Vec<i32> = Vec::with_capacity(queries.len());
    for query_node_val in queries {
        if let Some(node_i) = val_to_idx_map.get(&query_node_val).cloned() {
            ans_vec.push(node_info_vec[node_i]);
        } else {
            unreachable!()
        }
    }
    ans_vec
}
