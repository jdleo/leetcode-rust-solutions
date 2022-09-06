use std::borrow::{Borrow, BorrowMut};

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
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let (mut list1, mut list2) = (list1, list2);
        let mut dummy_head = Box::new(ListNode::new(0));
        let mut dummy = &mut dummy_head;

        // go while nodes to take
        while let (Some(head1), Some(head2)) = (list1.as_ref(), list2.as_ref()) {
            // take smaller node, and move that one forward
            if head1.val < head2.val {
                dummy.next = list1.take();
                dummy = dummy.next.as_mut().unwrap();
                list1 = dummy.next.take();
            } else {
                dummy.next = list2.take();
                dummy = dummy.next.as_mut().unwrap();
                list2 = dummy.next.take();
            }
        }

        // if either list1 or list2 still has nodes
        if list1.is_some() {
            dummy.next = list1.take();
        } else if list2.is_some() {
            dummy.next = list2.take();
        }

        dummy_head.next
    }
}
