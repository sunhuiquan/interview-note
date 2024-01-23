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
struct Solution;

use std::cell::RefCell;
use std::rc::Rc;

#[allow(dead_code)]
impl Solution {
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        if nums.is_empty() {
            return None;
        }
        Solution::sorted_array_to_bst_helper(&nums)
    }

    fn sorted_array_to_bst_helper(nums: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        let len = nums.len();
        let mid = len / 2;
        let node_rc = Rc::new(RefCell::new(TreeNode::new(nums[mid])));
        {
            let mut node = node_rc.borrow_mut();
            if mid > 0 {
                node.left = Solution::sorted_array_to_bst_helper(&nums[0..mid]);
            }
            if mid + 1 < len {
                node.right = Solution::sorted_array_to_bst_helper(&nums[mid + 1..]);
            }
        }
        Some(node_rc)
    }
}

fn main() {}
