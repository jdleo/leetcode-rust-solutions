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
    pub fn is_subtree(
        root: Option<Rc<RefCell<TreeNode>>>,
        sub_root: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        if root.is_none() {
            return false;
        }
        // simply return whether it's subrooted at current root, or left or right
        Self::is_same_tree(root.clone(), sub_root.clone())
            || Self::is_subtree(
                root.clone().unwrap().borrow().left.clone(),
                sub_root.clone(),
            )
            || Self::is_subtree(
                root.clone().unwrap().borrow().right.clone(),
                sub_root.clone(),
            )
    }

    // helper method to recursively calculate if same tree
    fn is_same_tree(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        // if either are null, they better be both null (to be same)
        if root1.is_none() || root2.is_none() {
            root1.is_none() && root2.is_none()
        } else {
            // both values must be same, and also left and right must be same
            let (root1, root2) = (root1.unwrap(), root2.unwrap());
            let (root1, root2) = (root1.borrow(), root2.borrow());
            root1.val == root2.val
                && Self::is_same_tree(root1.left.clone(), root2.left.clone())
                && Self::is_same_tree(root1.right.clone(), root2.right.clone())
        }
    }
}
