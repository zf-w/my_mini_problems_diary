//! ## Leetcode 2196. Create Binary Tree From Descritpions
//! https://leetcode.com/problems/create-binary-tree-from-descriptions/
//! - `Medium`; `Independently Solved`; `2024-07-15`;

use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::helpers::lc::TreeNode;

pub fn create_binary_tree(descriptions: Vec<Vec<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
    let mut notes: HashMap<i32, (Rc<RefCell<TreeNode>>, bool)> = HashMap::new();
    #[inline]
    fn parse_description(descritpion_ref: &Vec<i32>) -> (i32, i32, bool) {
        (
            descritpion_ref[0],
            descritpion_ref[1],
            descritpion_ref[2] == 1,
        )
    }

    for description_ref in descriptions.iter() {
        let (parent_i, child_i, is_left) = parse_description(description_ref);
        let parent_rc_mut_ref = notes
            .entry(parent_i)
            .or_insert((Rc::new(RefCell::new(TreeNode::new(parent_i))), false))
            .0
            .clone();
        let child_rc = notes
            .entry(child_i)
            .and_modify(|(_, has_parent_flag_mut_ref)| *has_parent_flag_mut_ref = true)
            .or_insert((Rc::new(RefCell::new(TreeNode::new(child_i))), true))
            .0
            .clone();
        if is_left {
            parent_rc_mut_ref.borrow_mut().left = Some(child_rc);
        } else {
            parent_rc_mut_ref.borrow_mut().right = Some(child_rc);
        }
    }

    for (_, (node_rc, has_parent_flag_ref)) in notes.iter() {
        if *has_parent_flag_ref == false {
            return Some(node_rc.clone());
        }
    }

    None
}
