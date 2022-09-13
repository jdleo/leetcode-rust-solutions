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
    pub fn insert_into_bst(
        root: Option<Rc<RefCell<TreeNode>>>,
        val: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        // null check
        if let Some(root) = root {
            // if equal or less, insert in left subtree, otherwise insert right
            if val <= root.borrow().val {
                let left = Self::insert_into_bst(root.borrow().left.clone(), val);
                root.borrow_mut().left = left;
            } else {
                let right = Self::insert_into_bst(root.borrow().right.clone(), val);
                root.borrow_mut().right = right;
            }

            Some(root)
        } else {
            // we found where we are supposed to insert (it fits here)
            // so we can create new node with val
            Some(Rc::new(RefCell::new(TreeNode::new(val))))
        }
    }
}
