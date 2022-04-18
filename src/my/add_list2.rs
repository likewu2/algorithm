use std::fmt::Debug;
use core::cmp::{PartialEq,Eq};

//Definition for a binary tree node.
type ListNodePtr<T>=Option<Box<ListNode<T>>>;

#[derive(Debug, PartialEq, Eq)]
pub struct ListNode<T> {
  pub val: T,
  pub next: ListNodePtr<T>,
}

impl<T> ListNode<T> {
  #[inline]
  pub fn new(val: T) -> Self {
    Self {
      val,
      next: None
    }
  }

  pub fn from_vec(vec: Vec<T>) -> ListNodePtr<T> {
    let mut nums = vec;
    nums.reverse();
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

  pub fn to_vec(head: ListNodePtr<T>) -> Vec<T> {
    let mut vec = vec![];
    let mut p = head;
    while let Some(t) = p {
      vec.push(t.val);
      p = t.next;
    }
    vec
  }
}


pub fn add_two_lists<T>(l1: ListNodePtr<T>, l2: ListNodePtr<T>) -> ListNodePtr<T> {
    match (l1, l2) {
        (Some(n1), Some(n2)) => {
          Some(n1)
        }
        (Some(n1), None) => Some(n1),
        (None, Some(n2)) => Some(n2),
        _ => None
    }
}
