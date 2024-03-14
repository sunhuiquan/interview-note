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
    pub fn add_one_row(
        root: Option<Rc<RefCell<TreeNode>>>,
        val: i32,
        depth: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if depth == 1 {
            return Some(Rc::new(RefCell::new(TreeNode {
                val,
                left: root,
                right: None,
            })));
        }

        let mut queue = VecDeque::new();
        queue.push_back(root.as_ref()?.clone());

        let mut layer = 0;
        while !queue.is_empty() {
            let n = queue.len();
            if layer + 1 == depth {
                for _ in 0..n {
                    // todo
                }
            }
            layer += 1;

            for _ in 0..n {
                let node = queue.pop_front().unwrap();
                let node_ref = node.borrow();
                if let Some(left) = node_ref.left.clone() {
                    queue.push_back(left);
                }
                if let Some(right) = node_ref.right.clone() {
                    queue.push_back(right);
                }
            }
        }
        root
    }
}

#[test]
fn test() {}
