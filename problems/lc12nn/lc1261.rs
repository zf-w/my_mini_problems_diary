//! # Leetcode 1261. Find Elements in a Contaminated Binary Tree
//! https://leetcode.com/problems/find-elements-in-a-contaminated-binary-tree/
//! - `Medium`; `y2025m02d21`; `Independently Solved`; `0ms`; `5.8ms`; `1 attempt`;
//!
//! Topic: tree/binary_tree.

use std::{cell::RefCell, collections::HashSet, rc::Rc};

use learn_cs::mod_helpers::lc_mod::TreeNode;

struct FindElements {
    hash_set: HashSet<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl FindElements {
    fn helper(
        curr_val: i32,
        hash_set_mut_ref: &mut HashSet<i32>,
        curr_root_rc: Rc<RefCell<TreeNode>>,
    ) {
        let curr_root_ref = curr_root_rc.borrow();

        hash_set_mut_ref.insert(curr_val);

        if let Some(left_subroot_rc) = curr_root_ref.left.clone() {
            Self::helper(curr_val * 2 + 1, hash_set_mut_ref, left_subroot_rc);
        }

        if let Some(right_subroot_rc) = curr_root_ref.right.clone() {
            Self::helper(curr_val * 2 + 2, hash_set_mut_ref, right_subroot_rc);
        }
    }

    pub fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        let mut hash_set: HashSet<i32> = HashSet::new();
        let root_rc = if let Some(root_rc) = root {
            root_rc
        } else {
            return Self { hash_set };
        };

        hash_set.insert(0);
        Self::helper(0, &mut hash_set, root_rc);

        Self { hash_set }
    }

    pub fn find(&self, target: i32) -> bool {
        self.hash_set.contains(&target)
    }
}
