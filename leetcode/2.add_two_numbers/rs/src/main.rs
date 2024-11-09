/// https://leetcode.com/problems/add-two-numbers/?envType=study-plan-v2&envId=top-interview-150

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
struct Solution;
impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut l1 = l1;
        let mut l2 = l2;
        let mut l3 = Some(Box::new(ListNode::new(0)));
        let mut l3_tail = l3.as_mut();
        let mut carry = 0;
        while l1.is_some() || l2.is_some() {
            let sum = carry
                + l1.as_ref().map_or(0, |node| node.val)
                + l2.as_ref().map_or(0, |node| node.val);
            carry = sum / 10;
            let node = ListNode::new(sum % 10);
            l3_tail.as_mut().unwrap().next = Some(Box::new(node));
            l3_tail = l3_tail.and_then(|node| node.next.as_mut());
            l1 = l1.and_then(|node| node.next);
            l2 = l2.and_then(|node| node.next);
        }
        if carry > 0 {
            l3_tail.as_mut().unwrap().next = Some(Box::new(ListNode::new(carry)));
        }
        l3.unwrap().next
    }
}

fn main() {
    println!("Hello, world!");
}
