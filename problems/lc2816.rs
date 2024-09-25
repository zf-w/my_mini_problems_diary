//! ## Leetcode 2816. Double a Number Represented as a Linked List
//! https://leetcode.com/problems/double-a-number-represented-as-a-linked-list
//! - `Medium`; `Independently Solved`; `2024-05-07`;
//!
//! Since this problem is only doubling the number by two, we only need to worry about one next digit. My idea is to check whether the first digit is larger than five or not and add one extra node accordingly. After viewing the solution, I noticed that it's possible to add a zero node in both cases and check whether the first digit is zero at the end. I guess that's a clever move in terms of coding complexity.

use std::ptr::addr_of_mut;

use crate::helpers::lc::ListNode;

pub fn double_it(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    if head.is_none() {
        return head;
    }
    let mut prev_val: &mut i32;

    let mut curr_node_opt: Option<*mut Box<ListNode>>;
    fn opt_mut_ref_to_opt_mut_ptr(curr: Option<&mut Box<ListNode>>) -> Option<*mut Box<ListNode>> {
        if let Some(node) = curr {
            Some(addr_of_mut!(*node))
        } else {
            None
        }
    }
    if head.as_ref().unwrap().val.clone() >= 5 {
        let curr_node = head.as_mut().unwrap();
        curr_node.val = curr_node.val * 2 % 10;
        prev_val = unsafe { addr_of_mut!(curr_node.val).as_mut().expect("Should have") };
        curr_node_opt = opt_mut_ref_to_opt_mut_ptr(curr_node.next.as_mut());
        let mut new_head = Box::new(ListNode::new(1));
        new_head.next = head;
        head = Some(new_head);
    } else {
        let curr_node = head.as_mut().unwrap();
        curr_node.val = curr_node.val * 2 % 10;
        prev_val = unsafe { addr_of_mut!(curr_node.val).as_mut().expect("Should have") };
        curr_node_opt = opt_mut_ref_to_opt_mut_ptr(curr_node.next.as_mut());
    }
    while let Some(node_mut_ptr) = curr_node_opt {
        let node_mut_ref = unsafe { node_mut_ptr.as_mut().expect("Should be a valid pointer") };

        let curr_val = node_mut_ref.val;
        if curr_val >= 5 {
            node_mut_ref.val += curr_val - 10;
            *prev_val += 1;
        } else {
            node_mut_ref.val += curr_val;
        }
        prev_val = unsafe {
            addr_of_mut!(node_mut_ref.val)
                .as_mut()
                .expect("Should have")
        };
        curr_node_opt = opt_mut_ref_to_opt_mut_ptr(node_mut_ref.next.as_mut());
    }
    head
}
