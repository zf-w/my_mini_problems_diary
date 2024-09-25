//! ## Leetcode 2807. Insert Greatest Common Divisors in Linked List
//! https://leetcode.com/problems/insert-greatest-common-divisors-in-linked-list/
//! - `Medium`; `Independently Solved`; `2024-09-10`;

use std::ptr::addr_of_mut;

use crate::helpers::lc::ListNode;
#[inline]
fn gcd(mut a: i32, mut b: i32) -> i32 {
    while a > 0 {
        let temp = a;
        a = b % a;
        b = temp;
    }
    b
}

pub fn insert_greatest_common_divisors(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    fn deref_ptr(node_mut_ptr: *mut ListNode) -> &'static mut ListNode {
        unsafe { node_mut_ptr.as_mut().expect("Dereferencing node") }
    }
    let mut curr_node_mut_ptr = if let Some(head_box_mut_ref) = head.as_mut() {
        addr_of_mut!(**head_box_mut_ref)
    } else {
        return head;
    };

    loop {
        let curr_node_mut_ref = deref_ptr(curr_node_mut_ptr);
        let mut next_node_box = if let Some(next_node_box) = curr_node_mut_ref.next.take() {
            next_node_box
        } else {
            break;
        };

        let mut to_insert_box =
            Box::new(ListNode::new(gcd(curr_node_mut_ref.val, next_node_box.val)));
        curr_node_mut_ptr = addr_of_mut!(*next_node_box);
        to_insert_box.next = Some(next_node_box);
    }
    head
}
