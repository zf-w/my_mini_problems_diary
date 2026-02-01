//! # Leetcode 865. Smallest Subtree with all the Deepest Nodes
//! https://leetcode.com/problems/smallest-subtree-with-all-the-deepest-nodes/
//! - `Medium`; `y2025m01d09`; `Independently Solved`; `0ms`; `2.2mb`; `1 attempt`;

pub fn subtree_with_all_deepest(
    root: Option<Rc<RefCell<TreeNode>>>,
) -> Option<Rc<RefCell<TreeNode>>> {
    fn helper(root_rc: Rc<RefCell<TreeNode>>) -> (usize, Rc<RefCell<TreeNode>>) {
        let l_opt = root_rc.borrow().left.clone();
        let r_opt = root_rc.borrow().right.clone();

        match (l_opt, r_opt) {
            (None, None) => (0, root_rc),
            (None, Some(r_node_rc)) => {
                let (r_depth, r_ans_rc) = helper(r_node_rc);
                (r_depth + 1, r_ans_rc)
            }
            (Some(l_node_rc), None) => {
                let (l_depth, l_ans_rc) = helper(l_node_rc);
                (l_depth + 1, l_ans_rc)
            }
            (Some(l_node_rc), Some(r_node_rc)) => {
                let (r_depth, r_ans_rc) = helper(r_node_rc);
                let (l_depth, l_ans_rc) = helper(l_node_rc);

                if r_depth == l_depth {
                    (r_depth + 1, root_rc)
                } else if r_depth > l_depth {
                    (r_depth + 1, r_ans_rc)
                } else {
                    (l_depth + 1, l_ans_rc)
                }
            }
        }
    }

    let root_rc = if let Some(root_rc) = root {
        root_rc
    } else {
        return None;
    };

    Some(helper(root_rc).1)
}