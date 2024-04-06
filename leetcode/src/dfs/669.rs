struct Solution {}
fn main() {}

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
    pub fn trim_bst(
        root: Option<Rc<RefCell<TreeNode>>>,
        low: i32,
        high: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let root = root?;
        let val = root.borrow().val;
        if val < low {
            Solution::trim_bst(root.borrow_mut().right.take(), low, high)
        } else if val > high {
            Solution::trim_bst(root.borrow_mut().left.take(), low, high)
        } else {
            let left = Solution::trim_bst(root.borrow_mut().left.take(), low, high);
            let right = Solution::trim_bst(root.borrow_mut().right.take(), low, high);
            root.borrow_mut().left = left;
            root.borrow_mut().right = right;
            Some(root)
        }
    }
}

#[test]
fn test() {}
