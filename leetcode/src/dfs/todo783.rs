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
use std::cmp::min;
use std::rc::Rc;
impl Solution {
    pub fn min_diff_in_bst(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return i32::MAX;
        }

        min(
            min(
                Solution::min_diff_in_bst_in_node(root.clone()),
                Solution::min_diff_in_bst(root.clone().unwrap().borrow().left.clone()),
            ),
            Solution::min_diff_in_bst(root.clone().unwrap().borrow().right.clone()),
        )
    }

    fn min_diff_in_bst_in_node(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut min = i32::MAX;
        if root.is_none() {
            return min;
        }
        let root = root.unwrap().clone();
        let val = root.borrow().val;

        if root.borrow().left.is_some() {
            let mut node: Rc<RefCell<TreeNode>> = root.borrow().left.clone().unwrap();
            while node.borrow().right.is_some() {
                let right = node.borrow().right.clone().unwrap();
                node = right;
            }
            let diff = (node.borrow().val - val).abs();
            if min > diff {
                min = diff
            };
        }

        if root.borrow().right.is_some() {
            let mut node = root.borrow().right.clone().unwrap();
            while node.borrow().left.is_some() {
                let left = node.borrow().left.clone().unwrap();
                node = left;
            }
            let diff = (node.borrow().val - val).abs();
            if min > diff {
                min = diff
            };
        }

        min
    }
}

#[test]
fn test() {}
