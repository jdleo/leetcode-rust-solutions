use std::borrow::Borrow;

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
    pub fn get_decimal_value(head: Option<Box<ListNode>>) -> i32 {
        let mut result: i32 = 0;
        let mut current = head.clone();

        // keep going while nodes
        while current.is_some() {
            let current_ref = current.to_owned().unwrap();
            // left shift result and add this bit
            result <<= 1;
            result |= current_ref.val;

            // keep going
            current = current_ref.next;
        }

        result
    }
}
