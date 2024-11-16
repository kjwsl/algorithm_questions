use std::mem;
use std::rc::Rc;

pub struct Solution;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
impl Solution {
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut current = head;
        let head = &current;
        let mut size = 0usize;

        while let Some(node) = current {
            current = node.next;
        }
    }
}

fn main() {
    todo!();
}
