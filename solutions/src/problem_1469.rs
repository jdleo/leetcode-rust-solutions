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
    pub fn get_lonely_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        // stack for dfs and result
        let mut result: Vec<i32> = Vec::new();
        let mut stack = VecDeque::from([root.unwrap()]);

        // go until stack empty
        while !stack.is_empty() {
            /*
            4 scenarios
            1. both some (no lonelys, push both to stack)
            2. left some (lonely, push left)
            3. right some (lonely, push right)
            4. both none (do nothing)
            */
            if let Some(current) = stack.pop_back() {
                let current = current.borrow();
                match (&current.left, &current.right) {
                    (Some(left), Some(right)) => {
                        stack.push_back(left.clone());
                        stack.push_back(right.clone());
                    }
                    (Some(left), None) => {
                        stack.push_back(left.clone());
                        result.push(left.borrow().val);
                    }
                    (None, Some(right)) => {
                        stack.push_back(right.clone());
                        result.push(right.borrow().val);
                    }
                    (None, None) => continue,
                }
            }
        }

        result
    }
}
