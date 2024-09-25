//! Leetcode 1669. Merge In Between Linked Lists
//! https://leetcode.com/problems/merge-in-between-linked-lists
//! - `Medium`; `Independently Solved`; `2024-03-19`;
//!
//! I learned the Rust version from solutions. I independently solved this problem via C++.
//!
//! Learning how the ownership system works with linked lists. I wonder why the Tree Node doesn't use Box.

use std::mem;

use crate::helpers::lc::ListNode;

pub fn merge_in_between(
    list1: Option<Box<ListNode>>,
    a: i32,
    b: i32,
    mut list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut head = list1.expect("Have head");
    let mut curr = head.as_mut();
    for _ in 0..(a - 1) {
        curr = curr.next.as_mut().expect("a is valid");
    }

    mem::swap(&mut curr.next, &mut list2);

    while curr.next.is_some() {
        curr = curr.next.as_mut().expect("list2 has some");
    }

    for _ in a..=b {
        list2 = list2.expect("valid a and b").next; // Delete
    }

    curr.next = list2;

    Some(head)
}
