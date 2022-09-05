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
    pub fn deepest_leaves_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        // create queue for bfs
        let mut queue = VecDeque::new();

        // only if root is not null
        if let Some(r) = root {
            queue.push_back(r);
        }

        let mut res: i32 = 0;

        // go while queue is non empty
        while !queue.is_empty() {
            // reset sum (because this is a new deeper level), and get size of level
            res = 0;
            let size = queue.len();

            // iterate thru each node in level
            for _ in 0..size {
                if let Some(cur) = queue.pop_front() {
                    res += cur.borrow().val;
                    // check if left is not null
                    if let Some(left) = cur.borrow_mut().left.take() {
                        queue.push_back(left);
                    }

                    // check if right is not null
                    if let Some(right) = cur.borrow_mut().right.take() {
                        queue.push_back(right);
                    }
                }
            }
        }

        res
    }
}
