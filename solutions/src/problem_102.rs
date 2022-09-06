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
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        // edge case: empty
        if root.is_none() {
            return Vec::new();
        }

        let mut levels: Vec<Vec<i32>> = Vec::new();
        let mut queue: VecDeque<Rc<RefCell<TreeNode>>> = VecDeque::from([root.unwrap()]);

        // bfs
        while !queue.is_empty() {
            // size of current level, and current level
            let size = queue.len();
            let mut current_level: Vec<i32> = Vec::new();

            for _ in 0..size {
                if let Some(current) = queue.pop_front() {
                    // popleft and add to current level
                    let current = current.borrow();
                    current_level.push(current.val);

                    // push children if any
                    if let Some(left) = &current.left {
                        queue.push_back(left.clone());
                    }

                    if let Some(right) = &current.right {
                        queue.push_back(right.clone());
                    }
                }
            }

            // push current level
            levels.push(current_level);
        }

        levels
    }
}
