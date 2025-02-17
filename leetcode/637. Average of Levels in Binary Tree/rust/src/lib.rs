pub struct Solution;
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
    pub fn average_of_levels(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<f64> {
        if root.is_none() {
            return Vec::new();
        }

        let mut sum: Vec<f64> = Vec::new();
        let mut count: Vec<i32> = Vec::new();

        Self::average_of_levels_helper(root, 0, &mut sum, &mut count);
        let ans = sum.iter()
            .zip(count.iter())
            .map(|(&sum, &cnt)| sum / cnt as f64)
            .collect();

        println!("{:?}", sum);
        println!("{:?}", count);

        ans
    }

    fn average_of_levels_helper(
        root: Option<Rc<RefCell<TreeNode>>>,
        level: i32,
        sum: &mut Vec<f64>,
        count: &mut Vec<i32>,
    ) {
        if root.is_none() {
            return;
        }
        if sum.len() <= level as usize {
            sum.push(0.0);
            count.push(0);
        }

        let root = root.unwrap();
        sum[level as usize] += root.borrow().val as f64;
        count[level as usize] += 1;

        Self::average_of_levels_helper(root.borrow().left.clone(), level + 1, sum, count);
        Self::average_of_levels_helper(root.borrow().right.clone(), level + 1, sum, count);
    }
}
