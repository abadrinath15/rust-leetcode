use crate::solution_files::Solution;

///Definition for singly-linked list.
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
        if head.is_none() {
            return head;
        }
        let mut head_return = head;
        let mut node = head_return.as_mut().unwrap();
        while let Some(next_node) = node.next.as_mut() {
            if next_node.val == node.val {
                node.next = next_node.next.take();
            } else {
                node = node.next.as_mut().unwrap();
            }
        }
        head_return
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_83_1() {
        let arr = [1, 1, 2];
        let mut head = Some(Box::new(ListNode::new(arr[0])));
        let mut head_ref = &mut head;
        for val in arr.iter().skip(1) {
            head_ref.as_mut().unwrap().next = Some(Box::new(ListNode::new(*val)));
            head_ref = &mut head_ref.as_mut().unwrap().next;
        }
        println!("{:?}", head);
        let res = Solution::delete_duplicates(head);
        println!("{:?}", res);
        panic!("Let's see...");
    }

    #[test]
    fn test_2() {}
}
