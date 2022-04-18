#![feature(is_sorted)]

use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;

use algo::my::{self,tree_sum::TreeNode,merge_list,add_list,add_list::ListNodePtr};

#[test]
fn small_tree_sum() {
    let a = Rc::new(RefCell::new(TreeNode::new(11)));
    let b = Rc::new(RefCell::new(TreeNode::new(0)));
    let c = Rc::new(RefCell::new(TreeNode::new(1)));
    
    a.borrow_mut().left=Some(b.clone());
    a.borrow_mut().right=Some(c.clone());
    //println!("a: {:?} *a: {:?}", a, *a);

    //let d = RefCell::new(Rc::new(TreeNode::new(1)));
    //println!("d: {:?}", d.borrow_mut());

    //println!("a: {}, b: {}, c: {}", Rc::strong_count(&a), Rc::strong_count(&b), Rc::strong_count(&c));
    let sum=my::tree_sum::sum_root_to_leaf2(Some(a));
    println!("sum: {:?}", sum);
}

#[test]
fn small_merge_two_lists() {
    let mut a = Box::new(merge_list::ListNode::new(6));
    let mut b = Box::new(merge_list::ListNode::new(14));
    let mut c = Box::new(merge_list::ListNode::new(17));
    b.next=Some(c);
    a.next=Some(b);

    let mut h = Box::new(merge_list::ListNode::new(11));
    let mut i = Box::new(merge_list::ListNode::new(24));
    h.next=Some(i);

    if let Some(sum)=my::merge_list::merge_two_lists(Some(a), Some(h)) {
      println!("sum: {:?}", sum);
    }
}


#[test]
fn small_add_two_lists() {
    let mut a = add_list::ListNode::from_vec(vec![1,2,3,4]);
    let mut b = add_list::ListNode::from_vec(vec![2,3,4]);

    let sum = add_list::ListNode::to_vec(my::add_list::add_two_lists(a, b));
    println!("sum: {:?}", sum);
}


#[test]
fn small_intersect_list() {
    let mut a = vec![1, 4, 5, 7, 11, 3, 9, 13];
    let mut b = vec![4, 7, 10, 8, 2, 11];

    if let Some(intersect)=my::intersect_list2::intersect_list(Some(a), Some(b)) {
      println!("sum: {:?}", intersect);
    }
}


#[test]
fn test11() {
  let mut a=[1,2,3,4];
  a[0]=11;
  //a[4]=21;
  println!("{:?}", a);
}

#[test]
fn test12() {
  let mut a=RefCell::new([1,2,3,4]);
  a.borrow_mut()[0]=11;
  //a.borrow_mut()[4]=21;
  println!("{:?}", a);
}

#[test]
fn test13() {
  let mut m0: HashMap<i32, i32> = HashMap::new();
  m0.insert(2, 1);
  println!("{:?}", m0);
}
