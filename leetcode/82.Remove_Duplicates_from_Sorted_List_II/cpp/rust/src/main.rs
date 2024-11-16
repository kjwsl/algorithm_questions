struct Solution;
//
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
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut ret = None;
        let mut tail = &mut ret;
        let mut to_delete = 200i32;

        while let Some(mut node) = head {
            head = node.next.take();
            if node.val == to_delete {
                continue;
            }
            if head.as_ref().map_or(false, |n| n.val == node.val) {
                to_delete = node.val;
                continue;
            }
            *tail = Some(node);
            tail = &mut tail.as_mut().unwrap().next;
        }

        ret
    }
}

fn main() {
    todo!();
}
