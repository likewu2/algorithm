use std::fmt::Debug;
use core::cmp::{PartialEq,Eq};

//Definition for a binary tree node.
pub type ListNodePtr=Option<Box<ListNode>>;

#[derive(Debug, PartialEq, Eq)]
pub struct ListNode {
  pub val: i32,
  pub next: ListNodePtr,
}

impl ListNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    Self {
      val,
      next: None
    }
  }

  pub fn from_vec(vec: Vec<i32>) -> ListNodePtr {
    let mut nums = vec;
    //nums.reverse();
    let mut p = None;
    for (i, item) in nums.iter().enumerate() {
      if i == nums.len() - 1 {
        let mut head = ListNode::new(*item);
        head.next = p;
        return Some(Box::new(head));
      } else {
        let mut node = ListNode::new(*item);
        node.next = p;
        p = Some(Box::new(node));
      }
    }
    None
  }

  pub fn to_vec(head: ListNodePtr) -> Vec<i32> {
    let mut vec = vec![];
    let mut p = head;
    while let Some(t) = p {
      vec.push(t.val);
      p = t.next;
    }
    vec.reverse();
    vec
  }
}


pub fn add_two_lists(l1: ListNodePtr, l2: ListNodePtr) -> ListNodePtr {
    match (l1, l2) {
        (Some(n1), Some(n2)) => {
          let mut node = Box::new(ListNode::new(n1.val+n2.val));
          node.next = add_two_lists(n1.next,n2.next);
          Some(node)
        }
        (Some(n1), None) => Some(n1),
        (None, Some(n2)) => Some(n2),
        _ => None
    }
}
