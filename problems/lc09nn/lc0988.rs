//! ## Leetcode 988. Smallest String Starting From Leaf
//! https://leetcode.com/problems/smallest-string-starting-from-leaf
//! - `Medium`; `Independently Solved`; `2024-04-16`;
//!
//! The key part of this problem is tracking the current path. Greedily comparing the current String from a leaf to an intermediate internal node is not enough because the heights of subtrees are different, and the sub-results might not be part of the smallest one. I think it's possible to use another tree to reuse the intermediate `char`s instead of storing them in multiple `String`s.

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

use crate::helpers::lc::TreeNode;

fn helper(root: *const TreeNode, curr: &mut VecDeque<char>) -> String {
    let root_ref = unsafe { root.as_ref().expect("Dereferencing root") };

    let curr_char = (root_ref.val as u8 + 97) as char;
    curr.push_front(curr_char);
    let l_string = if let Some(left_rc) = &root_ref.left {
        Some(helper(left_rc.as_ptr(), curr))
    } else {
        None
    };

    let r_string = if let Some(right_rc) = &root_ref.right {
        Some(helper(right_rc.as_ptr(), curr))
    } else {
        None
    };

    let ans = match (l_string, r_string) {
        (Some(l), Some(r)) => {
            if l < r {
                l
            } else {
                r
            }
        }
        (Some(l), None) => l,
        (None, Some(l)) => l,
        (None, None) => String::from_iter(curr.iter()),
    };
    curr.pop_front();
    ans
}

pub fn smallest_from_leaf(root: Option<Rc<RefCell<TreeNode>>>) -> String {
    if let Some(root_rc) = root {
        let mut path: VecDeque<char> = VecDeque::with_capacity(10);
        helper(root_rc.as_ptr(), &mut path)
    } else {
        "".to_string()
    }
}
