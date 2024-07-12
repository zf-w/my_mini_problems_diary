//! ## Leetcode 2058. Find the Minimum and Maximum Number of Nodes Between Critial Points
//! https://leetcode.com/problems/find-the-minimum-and-maximum-number-of-nodes-between-critical-points/
//! - `Medium`; `Independently Solved`; `2024-07-05`;

use crate::helpers::lc::ListNode;

pub fn nodes_between_critical_points(head: Option<Box<ListNode>>) -> Vec<i32> {
    let mut first_i_opt: Option<usize> = None;
    let mut prev_i_opt: Option<usize> = None;
    let mut prev_val_0_opt: Option<i32> = None;
    let mut prev_val_1_opt: Option<i32> = None;
    let mut curr_ref_opt = head.as_deref();
    let mut ans_min_dis = -1;
    let mut ans_max_dis = -1;
    let mut curr_i: usize = 0;
    while let Some(curr_node_ref) = curr_ref_opt {
        let curr_val = curr_node_ref.val;
        if let (Some(prev_val_0), Some(prev_val_1)) =
            (prev_val_0_opt.as_ref(), prev_val_1_opt.as_ref())
        {
            if (*prev_val_1 < *prev_val_0 && *prev_val_0 > curr_val)
                || (*prev_val_1 > *prev_val_0 && *prev_val_0 < curr_val)
            {
                if let Some(first_i) = first_i_opt.as_ref() {
                    ans_max_dis = (curr_i - *first_i) as i32;
                } else {
                    first_i_opt = Some(curr_i);
                }
                if let Some(prev_i) = prev_i_opt.as_ref() {
                    let curr_dis = (curr_i - *prev_i) as i32;
                    if ans_min_dis == -1 {
                        ans_min_dis = curr_dis;
                    } else {
                        ans_min_dis = ans_min_dis.min(curr_dis);
                    }
                }
                prev_i_opt = Some(curr_i);
            }
        }
        prev_val_1_opt = prev_val_0_opt;
        prev_val_0_opt = Some(curr_val);
        curr_ref_opt = curr_node_ref.next.as_deref();
        curr_i += 1;
    }
    vec![ans_min_dis, ans_max_dis]
}
