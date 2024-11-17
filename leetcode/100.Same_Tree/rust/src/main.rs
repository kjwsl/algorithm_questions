// Definition for a binary tree node.
struct Solution;

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
use std::borrow::Borrow;
use std::cell::{Ref, RefCell};
use std::ops::Deref;
use std::rc::Rc;
impl Solution {
    pub fn is_same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        if p.is_none() || q.is_none() {
            if p == q {
                return true;
            }
            return false;
        }

        let p = p.unwrap();
        let q = q.unwrap();

        let p = p.try_borrow();
        let q = q.try_borrow();

        if p.is_err() || q.is_err() {
            return false;
        }

        let p = p.unwrap();
        let q = q.unwrap();

        if p.val != q.val {
            return false;
        }
        if !Self::is_same_tree(p.left.clone(), q.left.clone()) {
            return false;
        }
        if !Self::is_same_tree(p.right.clone(), q.right.clone()) {
            return false;
        }

        true
    }
}
