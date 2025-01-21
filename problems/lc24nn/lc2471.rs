//! # Leetcode 2471. Minimum Number of Operations to Sort a Binary Tree by Level
//! https://leetcode.com/problems/minimum-number-of-operations-to-sort-a-binary-tree-by-level/
//! - `Medium`; `y2024m12d23`; `Independently Solved`; `27ms`; `11.9mb`; `1 attempt`;

pub fn minimum_operations(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let root_rc = if let Some(root_rc) = root {
        root_rc
    } else {
        return 0;
    };
    use std::collections::VecDeque;
    let mut node_queue: VecDeque<Rc<RefCell<TreeNode>>> = VecDeque::new();
    node_queue.push_back(root_rc);

    let mut node_values_vec: Vec<i32> = Vec::new();
    let mut node_idxs_vec: Vec<usize> = Vec::new();

    let mut ans_count = 0;

    while node_queue.is_empty() == false {
        let level_nodes_num = node_queue.len();
        if node_values_vec.capacity() < level_nodes_num {
            let additional_capacity = level_nodes_num - node_values_vec.capacity();
            node_values_vec.reserve(additional_capacity);
            node_idxs_vec.reserve(additional_capacity);
        }

        for i in 0..level_nodes_num {
            let curr_node_rc = node_queue.pop_front().expect("Should have enough.");
            let curr_node_borrow = curr_node_rc.borrow();
            node_values_vec.push(curr_node_borrow.val);
            node_idxs_vec.push(i);
            if let Some(left_rc) = curr_node_borrow.left.clone() {
                node_queue.push_back(left_rc);
            }
            if let Some(right_rc) = curr_node_borrow.right.clone() {
                node_queue.push_back(right_rc);
            }
        }
        node_idxs_vec.sort_by_key(|idx_ref| -> i32 { node_values_vec[*idx_ref] }); // Perhaps this way is more cache-friendly.

        // println!("{:?}", node_idxs_vec);
        let mut i: usize = 0;

        while i < level_nodes_num {
            let curr_elem_i = node_idxs_vec[i];
            if curr_elem_i == i {
                i += 1;
                continue;
            }
            node_idxs_vec.swap(i, curr_elem_i);
            ans_count += 1;
        }
        node_idxs_vec.clear();
        node_values_vec.clear();
    }
    ans_count
}
