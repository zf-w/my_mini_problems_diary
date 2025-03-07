//! ## Leetcode 2181. Merge Nodes in Between Zeros
//! https://leetcode.com/problems/merge-nodes-in-between-zeros/
//! - `Medium`; `Independently Solved`; `2024-07-04`;

use crate::helpers::lc::ListNode;

pub fn merge_nodes(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut head = if let Some(head_box) = head {
        head_box.next
    } else {
        return None;
    };
    let mut curr_node_opt = head.as_deref_mut();
    while let Some(curr_node) = curr_node_opt {
        if curr_node.val != 0 {
            let next_node_box_opt = curr_node.next.take();
            if let Some(next_node_box) = next_node_box_opt {
                if next_node_box.val != 0 {
                    curr_node.val += next_node_box.val;
                    curr_node.next = next_node_box.next;
                    curr_node_opt = Some(curr_node);
                } else {
                    curr_node.next = next_node_box.next;
                    curr_node_opt = curr_node.next.as_deref_mut();
                }
            } else {
                curr_node_opt = None;
            }
        } else {
            curr_node_opt = curr_node.next.as_deref_mut();
        }
    }
    head
}
