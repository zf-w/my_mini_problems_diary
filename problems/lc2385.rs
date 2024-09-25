//! ## Leetcode 1026. Maximum Difference Between Node And Ancestor
//! https://leetcode.com/problems/amount-of-time-for-binary-tree-to-be-infected
//! - `Medium`; `Indenpendently Solved`; `2024-01-09`;
//! 
//! I first turn the tree into a graph and then perform BFS on it.

use std::cell::RefCell;
use std::collections::{HashMap, HashSet, VecDeque};
use std::rc::Rc;

use crate::helpers::lc::TreeNode;

fn traverse(
    root_rc: Rc<RefCell<TreeNode>>,
    parent: Option<i32>,
    m: &mut HashMap<i32, [Option<i32>; 3]>,
) {
    let val = Some(root_rc.borrow().val);
    let mut adj: [Option<i32>; 3] = [parent, None, None];

    let left = root_rc.borrow().left.clone();
    let right = root_rc.borrow().right.clone();
    if let Some(left_rc) = left {
        adj[1] = Some(left_rc.borrow().val);
        traverse(left_rc.clone(), val, m);
    }

    if let Some(right_rc) = right {
        adj[2] = Some(right_rc.borrow().val);
        traverse(right_rc.clone(), val, m);
    }
    m.insert(val.unwrap(), adj);
}

pub fn amount_of_time(root: Option<Rc<RefCell<TreeNode>>>, start: i32) -> i32 {
    let mut m: HashMap<i32, [Option<i32>; 3]> = HashMap::new();
    let root_rc: Rc<RefCell<TreeNode>>;
    if let Some(curr_rc) = root {
        root_rc = curr_rc;
    } else {
        return 0;
    }
    let mut visited: HashSet<i32> = HashSet::new();
    traverse(root_rc, None, &mut m);
    let mut q: VecDeque<i32> = VecDeque::new();
    q.push_back(start);
    visited.insert(start);
    let mut ans = 0;
    while !q.is_empty() {
        let len = q.len();
        for _ in 0..len {
            let curr = q.pop_front().expect("Should have");
            let nexts = m.get(&curr).expect("Should have");
            for opt in nexts.iter() {
                if let Some(next) = opt {
                    if visited.contains(next) {
                        q.push_back(*next);
                        visited.insert(*next);
                    }
                }
            }
        }
        ans += 1;
    }
    ans - 1
}
