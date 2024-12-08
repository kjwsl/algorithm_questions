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
use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        if let Some(ref node) = root {
            let mut node = node.borrow();
            let mut left = node.left.clone();
            let right = node.right.clone();
            // Solution::flatten(&mut left);

            if let Some(left) = left.as_mut() {
                left.try_borrow().map(|mut left| {
                    while left.left.is_some() || left.right.is_some() {
                        if left.left.is_some() {
                            left = left.left.as_mut().unwrap().borrow_mut();
                        } else {
                            left = left.right.as_mut().unwrap().borrow_mut();
                        }
                    }
                    left.right = right;
                });
            }
        }
    }
}

fn main() {}
