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
    pub fn reverse_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut prev: Option<Box<ListNode>> = None;

        // go until the end of the list
        while let Some(mut node) = head {
            // save the next node
            let next = node.next.take();

            // set the previous node as the next node
            node.next = prev;

            // set the current node as the previous node
            prev = Some(node);

            // set the next node as the current node
            head = next;
        }

        // result is in prev
        prev
    }
}
