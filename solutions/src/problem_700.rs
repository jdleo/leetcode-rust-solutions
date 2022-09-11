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
    pub fn search_bst(
        root: Option<Rc<RefCell<TreeNode>>>,
        val: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        // if we ever reach null, we never found
        if let Some(root) = root {
            // check if found, should search left, or right
            if root.borrow().val == val {
                Some(root)
            } else if root.borrow().val < val {
                Self::search_bst(root.borrow().right.clone(), val)
            } else {
                Self::search_bst(root.borrow().left.clone(), val)
            }
        } else {
            None
        }
    }
}
