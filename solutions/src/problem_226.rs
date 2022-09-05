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
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        // null check
        if let Some(root) = root {
            // get left and right recursively
            let left = Self::invert_tree(root.borrow().left.clone());
            let right = Self::invert_tree(root.borrow().right.clone());

            // swap left and right
            root.borrow_mut().left = right;
            root.borrow_mut().right = left;

            Some(root)
        } else {
            None
        }
    }
}
