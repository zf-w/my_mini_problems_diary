//! # Leetcode 1022. Sum of Root To Leaf Binary Numbers
//! https://leetcode.com/problems/sum-of-root-to-leaf-binary-numbers/
//! - y2026m02d24; Independently Solved;

pub fn sum_root_to_leaf(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    fn sum_root_to_leaf_aux(
        root_rc: &Rc<RefCell<TreeNode>>,
        ans_sum_mut_ref: &mut i32,
        path_num: i32,
    ) {
        let root_val = root_rc.borrow().val;

        let next_path_num = (path_num << 1) + root_val;

        let mut leaf_flag = true;

        if let Some(left_rc) = &root_rc.borrow().left {
            leaf_flag = false;
            sum_root_to_leaf_aux(left_rc, ans_sum_mut_ref, next_path_num);
        }

        if let Some(right_rc) = &root_rc.borrow().right {
            leaf_flag = false;
            sum_root_to_leaf_aux(right_rc, ans_sum_mut_ref, next_path_num);
        }

        if leaf_flag == true {
            *ans_sum_mut_ref += next_path_num;
            // println!("{:b}", next_path_num);
        }
    }

    let root_rc = if let Some(root_rc) = root {
        root_rc
    } else {
        return 0;
    };

    let mut ans_sum = 0;

    sum_root_to_leaf_aux(&root_rc, &mut ans_sum, 0);

    ans_sum
}