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
    pub fn is_same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        // make sure both are Some
        if let (Some(p), Some(q)) = (p.clone(), q.clone()) {
            // values must be same, and left and right must be same
            p.borrow().val == q.borrow().val
                && Self::is_same_tree(p.borrow().left.clone(), q.borrow().left.clone())
                && Self::is_same_tree(p.borrow().right.clone(), q.borrow().right.clone())
        } else {
            // they better both be None
            p == q
        }
    }
}
