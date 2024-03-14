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
use std::collections::VecDeque;
use std::rc::Rc;

impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut ans = Vec::new();
        let mut q = VecDeque::new();
        if let Some(x) = root {
            q.push_back(x);
        }

        while !q.is_empty() {
            let n = q.len();
            let mut vals = Vec::with_capacity(n);
            for _ in 0..n {
                if let Some(node) = q.pop_front() {
                    let mut node_ref = node.borrow_mut();
                    vals.push(node_ref.val);

                    if let Some(left) = node_ref.left.take() {
                        q.push_back(left);
                    }
                    if let Some(right) = node_ref.right.take() {
                        q.push_back(right);
                    }
                }
            }
            ans.push(vals);
        }
        ans
    }
}

#[test]
fn test() {}
