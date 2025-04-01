//! ## Leetcode 951. Flip Equivalent Binary Trees
//! https://leetcode.com/problems/flip-equivalent-binary-trees/
//! - `Medium`; `Independently Solved`; `y2024m10d24`;

pub fn flip_equiv(
    root1: Option<Rc<RefCell<TreeNode>>>,
    root2: Option<Rc<RefCell<TreeNode>>>,
) -> bool {
    let (root_1_rc, root_2_rc) = match (root1, root2) {
        (Some(root_1_rc), Some(root_2_rc)) => (root_1_rc, root_2_rc),
        (None, None) => {
            return true;
        }
        _ => {
            return false;
        }
    };
    let root_1_borrow = root_1_rc.borrow();
    let root_2_borrow = root_2_rc.borrow();
    if root_1_borrow.val != root_2_borrow.val {
        return false;
    }
    (flip_equiv(root_1_borrow.left.clone(), root_2_borrow.left.clone())
        && flip_equiv(root_1_borrow.right.clone(), root_2_borrow.right.clone()))
        || (flip_equiv(root_1_borrow.left.clone(), root_2_borrow.right.clone())
            && flip_equiv(root_1_borrow.right.clone(), root_2_borrow.left.clone()))
}
