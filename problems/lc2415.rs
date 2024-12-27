//! # Leetcode 2415. Reverse Odd Levels of Binary Tree
//! https://leetcode.com/problems/reverse-odd-levels-of-binary-tree/
//! - `Medium`; `y2024m12d20`; `Independently Solved`; `4ms`; `3.6mb`; `1 attempt`;

pub fn reverse_odd_levels(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    let root_rc = if let Some(root_rc) = root {
        root_rc
    } else {
        return root;
    };

    let mut level_nodes: Vec<Rc<RefCell<TreeNode>>> = Vec::new();
    let mut curr_begin_i: usize = 0;
    level_nodes.push(std::rc::Rc::clone(&root_rc));

    while curr_begin_i < level_nodes.len() {
        let next_begin_i = level_nodes.len();
        for curr_i in curr_begin_i..level_nodes.len() {
            let curr_rc_ref = std::rc::Rc::clone(level_nodes.get(curr_i).expect("Should have"));
            let curr_rc_borrow_mut = curr_rc_ref.borrow_mut();
            match (
                curr_rc_borrow_mut.left.clone(),
                curr_rc_borrow_mut.right.clone(),
            ) {
                (Some(left_rc), Some(right_rc)) => {
                    level_nodes.push(left_rc);
                    level_nodes.push(right_rc);
                }
                _ => continue,
            };
        }
        curr_begin_i = next_begin_i;
    }

    let mut odd_level_flag = true;

    let mut curr_level_begin_i: usize = 1;
    let mut next_level_begin_i: usize = 3;
    while next_level_begin_i <= level_nodes.len() {
        if odd_level_flag {
            let mut begin_i = curr_level_begin_i;
            let mut end_i = next_level_begin_i - 1;
            while begin_i < end_i {
                let begin_rc = std::rc::Rc::clone(&level_nodes[begin_i]);
                let end_rc = std::rc::Rc::clone(&level_nodes[end_i]);

                std::mem::swap(&mut begin_rc.borrow_mut().val, &mut end_rc.borrow_mut().val);

                begin_i += 1;
                end_i -= 1;
            }
        }

        odd_level_flag = !odd_level_flag;

        curr_level_begin_i = next_level_begin_i;
        next_level_begin_i = next_level_begin_i * 2 + 1;
    }
    Some(root_rc)
}
