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
    pub fn evaluate_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        // safely unwrap
        if let Some(root) = root {
            let root_ref = root.borrow();
            // 0: false, 1: true, 2: or, 3: and
            match root.borrow().val {
                0 => false,
                1 => true,
                2 => {
                    Self::evaluate_tree(root_ref.left.clone())
                        || Self::evaluate_tree(root_ref.right.clone())
                }
                _ => {
                    Self::evaluate_tree(root_ref.left.clone())
                        && Self::evaluate_tree(root_ref.right.clone())
                }
            }
        } else {
            // assume true
            true
        }
    }
}
