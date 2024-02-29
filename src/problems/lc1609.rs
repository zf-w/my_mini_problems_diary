//! ## Leetcode 1609. Even Odd Tree
//! https://leetcode.com/problems/even-odd-tree
//! - `Medium`; `Independentlyl Solved`; `2024-02-29`;
//!
//! A BFS related problem. I guess, to some extent, level-related problems need BFS to be solved. Otherwise, the recursion would need a lot of information to check if the tree satisfies the criteria. I guess that approach is still approachable? You need to keep track of all the left-most and right-most nodes.

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

use crate::helpers::lc::TreeNode;

pub fn is_even_odd_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    let root_rc = if let Some(rc) = root {
        rc
    } else {
        return false;
    };
    let mut q: VecDeque<Rc<RefCell<TreeNode>>> = VecDeque::new();
    q.push_back(root_rc.clone());
    let mut need_even: bool = false;
    while !q.is_empty() {
        let len = q.len();
        let mut prev: Option<i32> = None;
        for _ in 0..len {
            let curr = q.pop_front().expect("Should have some");
            let curr_val = curr.borrow().val.clone();
            // print!("{} ", curr_val);
            if (curr_val % 2 == 0) == need_even {
                if let Some(prev_val) = prev {
                    if (prev_val > curr_val) != need_even || prev_val == curr_val {
                        return false;
                    }
                }

                if let Some(left) = &curr.borrow().left {
                    q.push_back(left.clone());
                }
                if let Some(right) = &curr.borrow().right {
                    q.push_back(right.clone());
                }
                prev = Some(curr_val);
            } else {
                return false;
            }
        }
        // println!("");
        need_even = !need_even;
    }
    true
}
