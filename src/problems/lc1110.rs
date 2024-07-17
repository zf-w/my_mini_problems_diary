//! ## Leetcode 1110. Delete Nodes And Return Forest
//! https://leetcode.com/problems/delete-nodes-and-return-forest/
//! - `Medium`; `Independently Solved`; `2024-07-17`;

use std::{cell::RefCell, collections::HashSet, rc::Rc};

use crate::helpers::lc::TreeNode;

fn helper(
    root_rc: Rc<RefCell<TreeNode>>,
    root_flag: bool,
    to_delete_set: &HashSet<i32>,
    ans_vec_mut_ref: &mut Vec<Option<Rc<RefCell<TreeNode>>>>,
) {
    let curr_val = root_rc.borrow().val.clone();

    if root_flag {
        ans_vec_mut_ref.push(Some(root_rc.clone()));
    }

    if to_delete_set.contains(&curr_val) {
        if let Some(left_rc) = root_rc.borrow_mut().left.take() {
            helper(left_rc, true, to_delete_set, ans_vec_mut_ref);
        }

        if let Some(right_rc) = root_rc.borrow_mut().right.take() {
            helper(right_rc, true, to_delete_set, ans_vec_mut_ref);
        }
    } else {
        if root_flag {
            ans_vec_mut_ref.push(Some(root_rc.clone()));
        }

        let left_opt = root_rc.borrow().left.clone();

        if let Some(left_rc) = left_opt {
            if to_delete_set.contains(&left_rc.borrow().val) {
                root_rc.borrow_mut().left = None;
            }
            helper(left_rc, false, to_delete_set, ans_vec_mut_ref);
        }

        let right_opt = root_rc.borrow().right.clone();

        if let Some(right_rc) = right_opt {
            if to_delete_set.contains(&right_rc.borrow().val) {
                root_rc.borrow_mut().right = None;
            }
            helper(right_rc, false, to_delete_set, ans_vec_mut_ref);
        }
    }
}

pub fn del_nodes(
    root: Option<Rc<RefCell<TreeNode>>>,
    to_delete: Vec<i32>,
) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
    let mut ans_vec: Vec<Option<Rc<RefCell<TreeNode>>>> = Vec::new();
    let to_delete_set = to_delete.iter().cloned().collect::<HashSet<i32>>();
    helper(
        root.expect("Non-empty tree"),
        true,
        &to_delete_set,
        &mut ans_vec,
    );
    ans_vec
}
