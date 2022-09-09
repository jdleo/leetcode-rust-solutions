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
    pub fn merge_trees(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        match (root1, root2) {
            (Some(root1), Some(root2)) => {
                // add and merge
                let (root1_ref, root2_ref) = (root1.borrow(), root2.borrow());
                let val = root1_ref.val + root2_ref.val;
                let mut root = TreeNode::new(val);
                root.left = Self::merge_trees(root1_ref.left.clone(), root2_ref.left.clone());
                root.right = Self::merge_trees(root1_ref.right.clone(), root2_ref.right.clone());
                Some(Rc::new(RefCell::new(root)))
            }
            (Some(root1), None) => {
                // simply clone left
                Some(root1)
            }
            (None, Some(root2)) => {
                // simply clone right
                Some(root2)
            }
            _ => None,
        }
    }
}
