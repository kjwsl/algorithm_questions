use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
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

fn print_tree(root: Option<Rc<RefCell<TreeNode>>>) {
    fn print_tree_rec(root: &Option<Rc<RefCell<TreeNode>>>, level: i32) {
        if let Some(node) = root {
            print_tree_rec(&node.borrow().right, level + 1);
            for _ in 0..level {
                print!("    ");
            }
            println!("{}", node.borrow().val);
            print_tree_rec(&node.borrow().left, level + 1);
        }
    }
    print_tree_rec(&root, 0);
}

impl Solution {
    fn build_tree_rec(
        preorder: &Vec<i32>,
        inorder: &Vec<i32>,
        pre_start: i32,
        in_start: i32,
        in_end: i32,
        inorder_map: &HashMap<i32, i32>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if pre_start >= preorder.len() as i32 || in_start > in_end {
            return None;
        }
        let root_val = preorder[pre_start as usize];
        let root = Rc::new(RefCell::new(TreeNode::new(root_val)));
        let in_root_index = inorder_map[&root_val];
        let left_size = in_root_index - in_start;
        let right_size = in_end - in_root_index;
        let left = Solution::build_tree_rec(
            preorder,
            inorder,
            pre_start + 1,
            in_start,
            in_root_index - 1,
            inorder_map,
        );
        let right = Solution::build_tree_rec(
            preorder,
            inorder,
            pre_start + left_size + 1,
            in_root_index + 1,
            in_end,
            inorder_map,
        );
        root.borrow_mut().left = left;
        root.borrow_mut().right = right;
        Some(root)
    }
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut inorder_map = HashMap::new();
        for (i, &val) in inorder.iter().enumerate() {
            inorder_map.insert(val, i as i32);
        }
        return Solution::build_tree_rec(
            &preorder,
            &inorder,
            0,
            0,
            inorder.len() as i32 - 1,
            &inorder_map,
        );
    }
}

fn main() {
    let preorder = vec![3, 9, 20, 15, 7];
    let inorder = vec![9, 3, 15, 20, 7];
    let root = Solution::build_tree(preorder, inorder);
    print_tree(root);

    let preorder = vec![-1];
    let inorder = vec![-1];
    let root = Solution::build_tree(preorder, inorder);
    print_tree(root);

    let preorder = vec![1, 2, 3];
    let inorder = vec![3, 2, 1];
    let root = Solution::build_tree(preorder, inorder);
    print_tree(root);

    let preorder = vec![1, 2, 3];
    let inorder = vec![1, 2, 3];
    let root = Solution::build_tree(preorder, inorder);
    print_tree(root);
}
