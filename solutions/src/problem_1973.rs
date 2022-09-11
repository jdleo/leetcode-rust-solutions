struct Solution;

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
    pub fn equal_to_descendants(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let (result, _) = Self::helper(root);
        result
    }

    // helper method to recursively keep track of result and sum of descendants
    fn helper(root: Option<Rc<RefCell<TreeNode>>>) -> (i32, i32) {
        // null safety
        if let Some(root) = root {
            // recursively calculate left and right
            let (left_result, left_sum) = Self::helper(root.borrow().left.clone());
            let (right_result, right_sum) = Self::helper(root.borrow().right.clone());

            // if equal to descendants, result + 1, also return sum of descendants
            (
                left_result + right_result + (left_sum + right_sum == root.borrow().val) as i32,
                left_sum + right_sum + root.borrow().val,
            )
        } else {
            (0, 0)
        }
    }
}
