//! # LeetCode 61. Rotate List
//! https://leetcode.com/problems/rotate-list/
//! - y2026m05d05; Independently Solved;

// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn rotate_right(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let mut list_len = 0;

        let mut curr_node_box_ref_opt = head.as_ref();

        while let Some(curr_node_box_ref) = curr_node_box_ref_opt {
            curr_node_box_ref_opt = curr_node_box_ref.next.as_ref();
            list_len += 1;
        }

        if list_len == 0 {
            return head;
        }

        let k = k % list_len;

        if k == 0 {
            return head;
        }

        let mut second_part_head_opt = head.as_mut();

        for _ in 0..(list_len - k - 1) {
            second_part_head_opt = second_part_head_opt.expect("len checked").next.as_mut();
        }

        let mut fisrt_part_head_opt = second_part_head_opt.expect("len checked...").next.take();

        let mut first_part_node_box_mut_ref_opt = fisrt_part_head_opt.as_mut();
        while let Some(first_part_node_box_mut_ref) = first_part_node_box_mut_ref_opt {
            if first_part_node_box_mut_ref.next.is_none() {
                first_part_node_box_mut_ref.next = head;
                break;
            }

            first_part_node_box_mut_ref_opt = first_part_node_box_mut_ref.next.as_mut();
        }

        fisrt_part_head_opt
    }
}