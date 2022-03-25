#![feature(is_sorted)]

use std::cell::RefCell;
use std::rc::Rc;

use algo::common;
use algo::my::{self,tree_sum::TreeNode};

#[test]
fn small_tree_sum() {
    let a = Rc::new(RefCell::new(TreeNode::new(1)));
    let b = Rc::new(RefCell::new(TreeNode::new(2)));
    let c = Rc::new(RefCell::new(TreeNode::new(3)));
    //a.borrow_mut().left=Some(b.clone());
    //a.borrow_mut().right=Some(c.clone());

    println!("a: {}, b: {}, c: {}", Rc::strong_count(&a), Rc::strong_count(&b), Rc::strong_count(&c));
    let sum=my::tree_sum::sum_root_to_leaf(Some(a));
    println!("sum: {:?}", sum);
}
