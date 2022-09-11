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
    pub fn find_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = Vec::new();
        let mut current_pruned: Vec<i32> = Vec::new();
        // keep going until tree is completely pruned
        // each iteration, will prune all the leaves
        let mut root = Self::prune(root, &mut current_pruned);
        while root.is_some() {
            // take current pruned, push and clear
            result.push(current_pruned);
            current_pruned = Vec::new();
            root = Self::prune(root, &mut current_pruned);
        }
        result.push(current_pruned);
        result
    }

    fn prune(
        root: Option<Rc<RefCell<TreeNode>>>,
        current_pruned: &mut Vec<i32>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        // null-safety
        if let Some(root) = root {
            // build left and right subtrees
            let left = Self::prune(root.borrow().left.clone(), current_pruned);
            let right = Self::prune(root.borrow().right.clone(), current_pruned);

            // check if current one is a leaf
            if root.as_ref().borrow().left.is_none() && root.as_ref().borrow().right.is_none() {
                // prune and add to pruned list
                (*current_pruned).push(root.borrow().val);
                None
            } else {
                // borrow as mutable and set left/right
                root.borrow_mut().left = left;
                root.borrow_mut().right = right;
                Some(root)
            }
        } else {
            None
        }
    }
}
