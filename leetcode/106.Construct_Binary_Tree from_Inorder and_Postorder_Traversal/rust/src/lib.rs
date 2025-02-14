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
use std::cmp::Ordering;
use std::collections::HashMap;
use std::rc::Rc;
impl Solution {
    pub fn build_tree(inorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut inorder_map = std::collections::HashMap::new();

        for (i, &val) in inorder.iter().enumerate() {
            inorder_map.insert(val, i);
        }

        let postorder_idx = postorder.len() - 1;
        Self::build_tree_helper(&inorder, &postorder, &inorder_map, postorder_idx, 0, inorder.len())
    }


    fn build_tree_helper(
        inorder: &Vec<i32>,
        postorder: &Vec<i32>,
        inorder_map: &HashMap<i32, usize>,
        postorder_index: usize,
        inorder_start: usize,
        inorder_end: usize,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if inorder_start >= inorder_end {
            return None;
        }

        let root_val = postorder[postorder_index];
        let root = Rc::new(RefCell::new(TreeNode::new(root_val)));

        if postorder_index == 0 {
            return Some(root);
        }

        let root_inorder_index = inorder_map[&root_val];
        let right_subtree_size = inorder_end - root_inorder_index - 1;

        if right_subtree_size > 0 {
            let right = Self::build_tree_helper(
                inorder,
                postorder,
                inorder_map,
                postorder_index - 1,
                root_inorder_index + 1,
                inorder_end,
            );
            root.borrow_mut().right = right;
        }

        if root_inorder_index > inorder_start {
            let left = Self::build_tree_helper(
                inorder,
                postorder,
                inorder_map,
                postorder_index - right_subtree_size - 1,
                inorder_start,
                root_inorder_index,
            );
            root.borrow_mut().left = left;
        }

        Some(root)
    }
}
