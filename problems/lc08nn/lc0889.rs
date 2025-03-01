//! # Leetcode 889. Construct Binary Tree from Preorder and Postorder Traversal
//! https://leetcode.com/problems/construct-binary-tree-from-preorder-and-postorder-traversal/
//! - `Medium`; `y2025m02d25`; `Independently Solved`; `0ms`; `2.2mb`; `1 attempt`;
//!
//! Topics: tree/binary_tree/traversal.

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn construct_from_pre_post(
        preorder: Vec<i32>,
        postorder: Vec<i32>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        fn find_index(iter: impl Iterator<Item = i32>, target_val: i32) -> usize {
            for (i, val) in iter.enumerate() {
                if val == target_val {
                    return i;
                }
            }
            unreachable!()
        }

        fn helper(preorder_arr_ref: &[i32], postorder_arr_ref: &[i32]) -> Rc<RefCell<TreeNode>> {
            // println!("{} {}", preorder_arr_ref.len(), postorder_arr_ref.len());
            assert_eq!(preorder_arr_ref.len(), postorder_arr_ref.len());

            let node_num = postorder_arr_ref.len();
            if node_num == 0 {
                unreachable!()
            }

            let mut root_node = TreeNode::new(preorder_arr_ref[0]);

            if preorder_arr_ref.len() == 1 {
                return Rc::new(RefCell::new(root_node));
            }

            let last_val_i = node_num - 1;

            let l_node_val = preorder_arr_ref[1];
            let r_node_val = postorder_arr_ref[last_val_i - 1];

            if l_node_val == r_node_val {
                let l_node_rc = helper(&preorder_arr_ref[1..], &postorder_arr_ref[..last_val_i]);

                root_node.left = Some(l_node_rc);
            } else {
                let pre_r_val_i = find_index(preorder_arr_ref[1..].iter().cloned(), r_node_val) + 1; // Critical bug here. I forgot that "slicing" resets array indices.
                let post_l_val_i =
                    find_index(postorder_arr_ref[..last_val_i].iter().cloned(), l_node_val);
                // println!("{}", pre_r_val_i);
                let l_node_rc = helper(
                    &preorder_arr_ref[1..pre_r_val_i],
                    &postorder_arr_ref[..=post_l_val_i],
                );
                let r_node_rc = helper(
                    &preorder_arr_ref[pre_r_val_i..],
                    &postorder_arr_ref[(post_l_val_i + 1)..last_val_i],
                );

                root_node.left = Some(l_node_rc);
                root_node.right = Some(r_node_rc);
            }

            Rc::new(RefCell::new(root_node))
        }

        if preorder.is_empty() == true {
            return None;
        }

        Some(helper(&preorder, &postorder))
    }
}
