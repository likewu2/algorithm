use std::rc::Rc;
use std::cell::RefCell;

use std::fmt::Debug;
use core::cmp::{PartialEq,Eq};

//Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    TreeNode {
      val,
      left: None,
      right: None
    }
  }
}


pub fn sum_root_to_leaf(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
  fn helper(root: Option<Rc<RefCell<TreeNode>>>, val: i32) -> i32 {
    match root {
        Some(node) => match node.borrow_mut() {
            mut p => match p.right.is_none() && p.left.is_none() {
              true => val | p.val,
              _ => helper(p.left.take(), (val | p.val) << 1) + helper(p.right.take(), (val | p.val) << 1)
            }
        }
        _ => 0
    }
  }
  helper(root, 0)
}
