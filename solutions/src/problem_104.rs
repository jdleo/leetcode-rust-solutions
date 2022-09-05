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
use std::cmp::max;
use std::rc::Rc;
impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        // null node?
        if let Some(root) = root {
            let root_ref = root.borrow();

            // calculate left and right
            let left = Solution::max_depth(root_ref.left.clone());
            let right = Solution::max_depth(root_ref.right.clone());

            // max of left and right (+ current depth)
            1 + max(left, right)
        } else {
            0
        }
    }
}
