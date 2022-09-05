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
use std::collections::VecDeque;
use std::rc::Rc;

impl Solution {
    pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        let mut k = k.clone();
        let mut current = root.clone();
        let mut stack: VecDeque<Option<Rc<RefCell<TreeNode>>>> = VecDeque::new();

        // go while stack or root are not empty
        while current.is_some() || !stack.is_empty() {
            // check if root is some
            if let Some(c) = current {
                // add to stack and keep going down left
                current = c.borrow().left.clone();
                stack.push_back(Some(c));
            } else {
                // pop from stack
                current = stack.pop_back().unwrap();

                // check if k'th smallest
                k -= 1;
                if k == 0 {
                    return current.unwrap().borrow().val;
                }

                // go right now
                current = current.unwrap().borrow().right.clone();
            }
        }

        0
    }
}
