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
use std::rc::Rc;
impl Solution {
    pub fn sum_numbers_rec(root: Option<Rc<RefCell<TreeNode>>>, sum: u32) -> u32 {
        if root.is_none() {
            return 0;
        }

        let left = root.as_ref().unwrap().borrow().left.clone();
        let right = root.as_ref().unwrap().borrow().right.clone();

        let mut sum = sum * 10 + root.as_ref().unwrap().borrow().val as u32;

        if left.is_none() && right.is_none() {
            return sum;
        }

        Solution::sum_numbers_rec(left, sum) + Solution::sum_numbers_rec(right, sum)
    }
    pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Solution::sum_numbers_rec(root, 0) as i32
    }
}
