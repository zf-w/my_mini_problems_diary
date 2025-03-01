//! # Leetcode 1028. Recover a Tree From Preorder Traversal
//! https://leetcode.com/problems/recover-a-tree-from-preorder-traversal/
//! - `Hard`; `y2025m02d28`; `Learned from Solution`; `0ms`; `2.4mb`; `1 attempt`;
//!
//! Topics: tree/binary_tree/traversal.
//!
//! Learned from https://leetcode.com/problems/recover-a-tree-from-preorder-traversal/solutions/6418089/recover-a-tree-from-preorder-traversal

use std::cell::RefCell;
use std::rc::Rc;

use learn_cs::mod_helpers::lc_mod::TreeNode;

pub fn helper(
    string_byte_arr_ref: &[u8],
    idx_mut_ref: &mut usize,
    depth: usize,
) -> Option<Rc<RefCell<TreeNode>>> {
    let len = string_byte_arr_ref.len();
    let begin_idx = *idx_mut_ref;
    let mut dash_count: usize = 0;
    while *idx_mut_ref < len && string_byte_arr_ref[*idx_mut_ref] == b'-' {
        dash_count += 1;
        *idx_mut_ref += 1;
    }

    if dash_count != depth {
        *idx_mut_ref = begin_idx;
        return None;
    }

    let mut node_val: i32 = 0;

    while *idx_mut_ref < len && string_byte_arr_ref[*idx_mut_ref].is_ascii_digit() {
        node_val = 10 * node_val + ((string_byte_arr_ref[*idx_mut_ref] - b'0') as i32);
        *idx_mut_ref += 1;
    }

    let mut tree_node: TreeNode = TreeNode::new(node_val);

    tree_node.left = helper(string_byte_arr_ref, idx_mut_ref, depth + 1);
    tree_node.right = helper(string_byte_arr_ref, idx_mut_ref, depth + 1);

    Some(Rc::new(RefCell::new(tree_node)))
}

pub fn recover_from_preorder(traversal: String) -> Option<Rc<RefCell<TreeNode>>> {
    let mut idx = 0;
    let string_byte_arr_ref = traversal.as_bytes();

    helper(string_byte_arr_ref, &mut idx, 0)
}
