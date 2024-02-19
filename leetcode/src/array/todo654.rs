struct Solution;
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

#[allow(dead_code)]
impl Solution {
    pub fn construct_maximum_binary_tree(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Solution::construct_maximum_binary_tree_helper(&nums)
    }

    fn construct_maximum_binary_tree_helper(nums: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        let max_tuple = nums.iter().enumerate().max_by_key(|&(_, item)| item);
        match max_tuple {
            Some((idx, &val)) => {
                let mut node = TreeNode::new(val);
                node.left = Solution::construct_maximum_binary_tree_helper(&nums[..idx]);
                node.right = Solution::construct_maximum_binary_tree_helper(&nums[idx + 1..]);
                Some(Rc::new(RefCell::new(node)))
            }
            None => None,
        }
    }
}

#[test]
fn test1() {
    let tree = Solution::construct_maximum_binary_tree(vec![3, 2, 1, 6, 0, 5]);
    println!("{:?}", tree);
}
