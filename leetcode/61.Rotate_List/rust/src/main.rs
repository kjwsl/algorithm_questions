use std::{collections::VecDeque, rc::Rc};

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
    pub fn rotate_right(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        head.as_ref()?;
        let mut head = head;
        let mut deq = VecDeque::new();
        let mut current = &head;
        let mut cnt = 0;
        while let Some(node) = current {
            deq.push_back(node.val);
            current = &node.next;
            cnt += 1;
        }

        let k = k % cnt;

        for _ in 0..k {
            if let Some(val) = deq.pop_back() {
                deq.push_front(val);
            }
        }

        let mut current = head.as_mut();
        while let Some(node) = current {
            if let Some(popped) = deq.pop_front() {
                node.val = popped;
            }

            current = node.next.as_mut();
        }

        head
    }
}
