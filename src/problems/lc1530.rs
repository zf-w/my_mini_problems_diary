//! ## Leetcode 1530. Number of Good Leaf Nodes Pairs
//! https://leetcode.com/problems/number-of-good-leaf-nodes-pairs/
//! - `Medium`; `Independently Solved`; `2024-07-18`;

use std::{cell::RefCell, rc::Rc};

use crate::helpers::lc::TreeNode;

fn helper(root_rc: Rc<RefCell<TreeNode>>, ans_count: &mut i32, limit: usize) -> Vec<usize> {
    let left_node_dists_opt: Option<Vec<usize>> =
        if let Some(left_rc) = root_rc.borrow().left.clone() {
            Some(helper(left_rc, ans_count, limit))
        } else {
            None
        };

    let right_node_dists_opt: Option<Vec<usize>> =
        if let Some(right_rc) = root_rc.borrow().right.clone() {
            Some(helper(right_rc, ans_count, limit))
        } else {
            None
        };

    match (left_node_dists_opt, right_node_dists_opt) {
        (None, None) => {
            vec![1]
        }
        (None, Some(mut node_dists)) => {
            for val_mut_ref in node_dists.iter_mut() {
                *val_mut_ref += 1;
            }
            node_dists
        }
        (Some(mut node_dists), None) => {
            for val_mut_ref in node_dists.iter_mut() {
                *val_mut_ref += 1;
            }
            node_dists
        }
        (Some(left_dists), Some(right_dists)) => {
            for left_dist_ref in left_dists.iter() {
                for right_dist_ref in right_dists.iter() {
                    if *left_dist_ref + *right_dist_ref <= limit {
                        *ans_count += 1;
                    }
                }
            }
            let mut ans_dists_vec: Vec<usize> =
                Vec::with_capacity(left_dists.len() + right_dists.len());
            for left_dist_ref in left_dists.iter() {
                if *left_dist_ref + 1 < limit {
                    ans_dists_vec.push(*left_dist_ref + 1);
                }
            }
            for right_dist_ref in right_dists.iter() {
                if *right_dist_ref + 1 < limit {
                    ans_dists_vec.push(*right_dist_ref + 1);
                }
            }
            ans_dists_vec
        }
    }
}

pub fn count_pairs(root: Option<Rc<RefCell<TreeNode>>>, distance: i32) -> i32 {
    let mut ans_count: i32 = 0;
    let limit: usize = distance as usize;
    let root_rc = root.expect("Non-empty");
    let left_node_dists_opt: Option<Vec<usize>> =
        if let Some(left_rc) = root_rc.borrow().left.clone() {
            Some(helper(left_rc, &mut ans_count, limit))
        } else {
            None
        };

    let right_node_dists_opt: Option<Vec<usize>> =
        if let Some(right_rc) = root_rc.borrow().right.clone() {
            Some(helper(right_rc, &mut ans_count, limit))
        } else {
            None
        };

    match (left_node_dists_opt, right_node_dists_opt) {
        (Some(left_dists), Some(right_dists)) => {
            for left_dist_ref in left_dists.iter() {
                for right_dist_ref in right_dists.iter() {
                    if *left_dist_ref + *right_dist_ref <= limit {
                        ans_count += 1;
                    }
                }
            }
        }
        _ => (),
    }
    ans_count
}
