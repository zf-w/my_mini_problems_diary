//! # Leetcode 1290. Convert Binary Number in a Linked List to Integer
//! https://leetcode.com/problems/convert-binary-number-in-a-linked-list-to-integer/
//! - `Easy`; `y2025m07d14`; `Independently Solved`; `0ms`; `2.2mb`; `1 attempt`;
//! Topics: linked_list.

pub fn get_decimal_value(head: Option<Box<ListNode>>) -> i32 {
    let mut ans_num = 0;

    let mut curr_node_ref_opt = head.as_ref();

    while let Some(curr_node_ref) = curr_node_ref_opt {
        ans_num = (ans_num << 1) + curr_node_ref.val;
        curr_node_ref_opt = curr_node_ref.next.as_ref();
    }

    ans_num
}