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
use std::cmp::Ordering::{Greater, Less};
use std::rc::Rc;
impl Solution {
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        // none-check
        if let Some(root) = root {
            /*
            3 scenarios:
            0: p and q are in left, search left
            1: p and q are in right, search right
            2: this is the LCA
            */
            let (pv, pq, root_ref) = (
                p.as_ref().unwrap().borrow().val,
                q.as_ref().unwrap().borrow().val,
                root.borrow(),
            );

            match (root.borrow().val.cmp(&pv), root.borrow().val.cmp(&pq)) {
                (Greater, Greater) => Self::lowest_common_ancestor(root_ref.left.clone(), p, q),
                (Less, Less) => Self::lowest_common_ancestor(root_ref.right.clone(), p, q),
                (_, _) => Some(root.clone()),
            }
        } else {
            None
        }
    }
}
