//! ## Leetcode 2641. Cousins in Binary Tree II
//! https://leetcode.com/problems/cousins-in-binary-tree-ii/
//! - `Medium`; `Independently Solved`; `y2024m10d23`;

use std::cell::RefCell;
use std::rc::Rc;

fn replace_value_in_tree_level_sum_helper(
    level_usize: usize,
    level_sums_vec_mut_ref: &mut Vec<i32>,
    sub_root: Rc<RefCell<TreeNode>>,
) {
    if level_sums_vec_mut_ref.len() == level_usize {
        level_sums_vec_mut_ref.push(0);
    }

    let sub_root_borrow = sub_root.borrow();
    level_sums_vec_mut_ref[level_usize] += sub_root_borrow.val as i32;
    let next_level_usize = level_usize + 1;
    if let Some(left_sub_root_rc) = sub_root_borrow.left.clone() {
        replace_value_in_tree_level_sum_helper(
            next_level_usize,
            level_sums_vec_mut_ref,
            left_sub_root_rc,
        );
    }
    if let Some(right_sub_root_rc) = sub_root_borrow.right.clone() {
        replace_value_in_tree_level_sum_helper(
            next_level_usize,
            level_sums_vec_mut_ref,
            right_sub_root_rc,
        );
    }
}

fn replace_value_in_tree_modify_helper(
    sub_root: Rc<RefCell<TreeNode>>,
    level_i: usize,
    sibling_sum: i32,
    level_sums_vec_ref: &Vec<i32>,
) {
    let curr_cousins_sum = level_sums_vec_ref[level_i] - sibling_sum;
    sub_root.borrow_mut().val = curr_cousins_sum;

    let mut next_sibling_sum = 0;
    let next_level_i = level_i + 1;

    let sub_root_borrow = sub_root.borrow();
    if let Some(left_rc) = &sub_root_borrow.left {
        next_sibling_sum += left_rc.borrow().val;
    }
    if let Some(right_rc) = &sub_root_borrow.right {
        next_sibling_sum += right_rc.borrow().val;
    }

    if let Some(left_rc) = &sub_root_borrow.left {
        replace_value_in_tree_modify_helper(
            Rc::clone(left_rc),
            next_level_i,
            next_sibling_sum,
            level_sums_vec_ref,
        );
    }
    if let Some(right_rc) = &sub_root_borrow.right {
        replace_value_in_tree_modify_helper(
            Rc::clone(right_rc),
            next_level_i,
            next_sibling_sum,
            level_sums_vec_ref,
        );
    }
}

pub fn replace_value_in_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    let root_rc = root?;
    let mut level_sums_vec: Vec<i32> = Vec::new();

    let root_val = root_rc.borrow().val.clone();

    replace_value_in_tree_level_sum_helper(0, &mut level_sums_vec, Rc::clone(&root_rc));

    replace_value_in_tree_modify_helper(Rc::clone(&root_rc), 0, root_val, &level_sums_vec);

    Some(root_rc)
}
