use core::panic;
use std::collections::VecDeque;

struct Solution;
// Definition for singly-linked list.
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
    pub fn partition(head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut current = head.as_ref();
        let mut small = VecDeque::new();
        let mut big = VecDeque::new();

        while let Some(node) = current {
            if node.val >= x {
                big.push_back(node.val);
            } else {
                small.push_back(node.val);
            }
            current = node.next.as_ref();
        }

        let mut current = head.as_mut();
        while let Some(node) = current {
            let val = if !small.is_empty() {
                small.pop_front().expect("No way")
            } else {
                big.pop_front().expect("no fucking way")
            };
            node.val = val;
            current = node.next.as_mut();
        }

        head
    }
}
