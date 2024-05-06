//! ## Leetcode 2487. Remove Nodes From Linked List
//! https://leetcode.com/problems/remove-nodes-from-linked-list
//! - `Medium`; `Independently Solved`; `2024-05-05`;
//!
//! The Rust ownership model makes making a reverse linked list a little bit harder. Considering the size of the problem, using an array vector of pointers would make the ownership model clearer. One funny bug occurred during the process. I accidentally used `to_owned` to move a box out of the nodes without realizing that it was actually copying the whole list out. I checked the C++ solution and felt my solution should work. Then, I found out the problem. It was hilarious.

use crate::helpers::lc::ListNode;

pub fn remove_nodes(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut nodes: Vec<Box<ListNode>> = Vec::with_capacity(1000);
    let mut curr_opt = head;
    while let Some(mut curr) = curr_opt {
        let next_opt = curr.next.take();
        // println!("{}", curr.val);
        curr.next = None;
        while !nodes.is_empty() && nodes.last().unwrap().val < curr.val {
            nodes.pop();
        }
        nodes.push(curr);
        curr_opt = next_opt;
    }
    let mut head = None;
    while let Some(mut last_box) = nodes.pop() {
        last_box.next = head;
        head = Some(last_box);
    }
    head
}
