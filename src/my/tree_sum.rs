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


pub fn sum_root_to_leaf1(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
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

pub fn sum_root_to_leaf2(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
  fn helper(root: Option<Rc<RefCell<TreeNode>>>, val: i32) -> i32 {
    match root {
        Some(node) => {
            //let ans = val * 10 + node.borrow().val;  //error
            let ans = val + node.borrow().val;
            if node.borrow().left.is_none() && node.borrow().right.is_none() {
              ans
            } else if node.borrow().left.is_none() {
              helper(node.borrow().right.clone(), ans)
            } else if node.borrow().right.is_none() {
              helper(node.borrow().left.clone(), ans)
            } else {
              //helper(node.borrow().left.clone(), ans) + helper(node.borrow().right.clone(), ans)
              helper(node.borrow().left.clone(), 0) + helper(node.borrow().right.clone(), 0) + ans
            }
        }
        _ => 0
    }
  }
  helper(root, 0)
}
