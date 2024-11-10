// Definition for singly-linked list.
struct Solution;

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
    pub fn reverse_between(
        head: Option<Box<ListNode>>,
        left: i32,
        right: i32,
    ) -> Option<Box<ListNode>> {
        // If the list is empty or no need to reverse, return the original list.
        if head.is_none() || left == right {
            return head;
        }

        // Create a dummy node to simplify edge cases where the head might be reversed.
        let mut dummy = Some(Box::new(ListNode { val: 0, next: head }));
        let mut prev = &mut dummy;

        // Move `prev` to the node just before the `left`-th node.
        for _ in 1..left {
            if let Some(ref mut node) = prev.as_mut().unwrap().next {
                prev = &mut prev.as_mut().unwrap().next;
            } else {
                // If `left` is out of bounds, return the original list.
                return dummy.unwrap().next;
            }
        }

        // `curr` will point to the `left`-th node.
        let mut curr = prev.as_mut().unwrap().next.take();
        let mut next = None;

        // Reverse the sublist from `left` to `right`.
        for _ in left..=right {
            if let Some(mut curr_node) = curr {
                let temp = curr_node.next.take();
                curr_node.next = next;
                next = Some(curr_node);
                curr = temp;
            } else {
                break; // If `right` exceeds the list length, stop reversing.
            }
        }

        // Connect the reversed sublist back to the list.
        prev.as_mut().unwrap().next = next;
        // Find the tail of the reversed sublist to connect with the remaining list.
        let mut tail = prev;
        for _ in left..=right {
            if let Some(ref mut node) = tail.as_mut().unwrap().next {
                tail = &mut tail.as_mut().unwrap().next;
            } else {
                break;
            }
        }
        tail.as_mut().unwrap().next = curr;

        // Return the modified list, excluding the dummy node.
        dummy.unwrap().next
    }
}

fn main() {
    todo!();
}
