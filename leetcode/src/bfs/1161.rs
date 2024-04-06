struct Solution {}
fn main() {}

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

#[allow(dead_code)]
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

#[allow(dead_code)]
impl Solution {
    pub fn max_level_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut q = std::collections::VecDeque::new();
        q.push_back(root.unwrap());

        let mut max_level = 0;
        let mut max = i32::MIN;
        let mut level = 1;
        while !q.is_empty() {
            let mut sum = 0;
            let size = q.len();
            for _ in 0..size {
                let node = q.pop_front().unwrap();
                let ref_node = node.borrow();
                sum += ref_node.val;

                if let Some(l) = ref_node.left.clone() {
                    q.push_back(l.clone());
                }
                if let Some(r) = ref_node.right.clone() {
                    q.push_back(r.clone());
                }
            }

            if sum > max {
                max = sum;
                max_level = level;
            }
            level += 1;
        }
        max_level
    }
}

#[test]
fn test() {}
