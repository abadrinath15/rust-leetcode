use crate::solution_files::Solution;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>,
}

impl ListNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    ListNode { next: None, val }
  }
}

impl Solution {
  pub fn merge_two_lists(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
  ) -> Option<Box<ListNode>> {
    let mut res_head = Box::new(ListNode::new(-101));
    let mut res = &mut res_head;
    let mut l1_mut = &l1;
    let mut l2_mut = &l2;
    let mut min_val: i32;
    if l1_mut.is_none() & l2_mut.is_none() {return None}
    while l1_mut.is_some() & l2_mut.is_some() {
      let l1_val = l1_mut.as_ref().unwrap();
      let l2_val = l2_mut.as_ref().unwrap();
      min_val = if l1_val.val < l2_val.val {
        l1_mut = &l1_val.next;
        l1_val.val
      }
      else {
        l2_mut = &l2_val.next;
        l2_val.val
      };
      match res.val {
        -101 => res.val = min_val,
        _ => {
          res.next = Some(Box::new(ListNode::new(min_val)));
          res = res.next.as_mut().unwrap();
        }
      };
    }
    match res.val {
      -101 => {
        if l1_mut.is_none() {
          res_head = l2_mut.as_ref().unwrap().clone();
        }
        else {
          res_head = l1_mut.as_ref().unwrap().clone();
        }
      },
      _ => {
        if l1_mut.is_none() {
          res.next = l2_mut.clone();
        }
        else {
          res.next = l1_mut.clone();
        }
      },
    };
    Some(res_head)
  }
}
