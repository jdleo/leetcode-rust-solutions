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
    pub fn sum_even_grandparent(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut result: i32 = 0;

        // start dfs
        Solution::dfs(root, -1, -1, &mut result);

        result
    }

    // recursive dfs helper method to sum all
    fn dfs(
        root: Option<Rc<RefCell<TreeNode>>>,
        parent_val: i32,
        grandparent_val: i32,
        result: &mut i32,
    ) {
        // if null root, don't do anything
        if let Some(root) = root {
            // check if grandparent value is even
            if (grandparent_val & 1 == 0) {
                *result += root.borrow().val;
            }

            // check left and right
            Solution::dfs(root.borrow().left, root.borrow().val, parent_val, result);
            Solution::dfs(root.borrow().right, root.borrow().val, parent_val, result);
        }
    }
}
