// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn middle_node(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut pointer = head.as_ref();
        let mut counter: i32 = 0;

        while let Some(node) = pointer {
            counter += 1;
            pointer = node.next.as_ref();
        }

        for _ in 0..counter / 2 {
            head = head.unwrap().next;
        }
        head
    }
}
