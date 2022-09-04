// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn range_sum_bst(root: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> i32 {
        // only take if root is non-null
        if let Some(root) = root {
            let root_ref = root.borrow();

            // check if lower than low (shouldnt search left anymore)
            if root_ref.val < low {
                Solution::range_sum_bst(root_ref.right.clone(), low, high)
            } else if root_ref.val > high {
                Solution::range_sum_bst(root_ref.left.clone(), low, high)
            } else {
                root_ref.val
                    + Solution::range_sum_bst(root_ref.right.clone(), low, high)
                    + Solution::range_sum_bst(root_ref.left.clone(), low, high)
            }
        } else {
            0
        }
    }
}
