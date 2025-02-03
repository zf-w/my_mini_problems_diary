//! ## Leetcode 2096. Step-By-Step Directions From a Binary Tree Node to Another
//! https://leetcode.com/problems/step-by-step-directions-from-a-binary-tree-node-to-another/
//! - `Medium`; `Independently Solved`; `2024-07-17`;

use std::{cell::RefCell, rc::Rc};

use crate::helpers::lc::TreeNode;

fn find_path_helper(
    root_rc: Rc<RefCell<TreeNode>>,
    path_mut_ref: &mut Vec<char>,
    target_val: i32,
) -> bool {
    if root_rc.borrow().val == target_val {
        return true;
    }

    if let Some(left_rc) = root_rc.borrow().left.clone() {
        path_mut_ref.push('L');
        if find_path_helper(left_rc, path_mut_ref, target_val) {
            return true;
        }
        path_mut_ref.pop();
    }

    if let Some(right_rc) = root_rc.borrow().right.clone() {
        path_mut_ref.push('R');
        if find_path_helper(right_rc, path_mut_ref, target_val) {
            return true;
        }
        path_mut_ref.pop();
    }

    false
}

fn find_path(root_rc: Rc<RefCell<TreeNode>>, target_val: i32) -> Option<Vec<char>> {
    let mut path: Vec<char> = Vec::new();

    if find_path_helper(root_rc, &mut path, target_val) {
        Some(path)
    } else {
        None
    }
}

pub fn get_directions(
    root: Option<Rc<RefCell<TreeNode>>>,
    start_value: i32,
    dest_value: i32,
) -> String {
    let start_path = find_path(
        root.clone().expect("At least two nodes in the tree"),
        start_value,
    )
    .expect("The tree should contain it");
    let dest_path = find_path(root.expect("At least two nodes in the tree"), dest_value)
        .expect("The tree should contain it");

    let start_path_len = start_path.len();
    let dest_path_len = dest_path.len();

    let mut i: usize = 0;
    while i < start_path_len && i < dest_path_len && start_path[i] == dest_path[i] {
        i += 1;
    }

    let mut ans_string: String = String::new();

    for _ in 0..(start_path.len() - i) {
        ans_string.push('U');
    }

    for j in i..dest_path_len {
        ans_string.push(dest_path[j]);
    }

    ans_string
}
