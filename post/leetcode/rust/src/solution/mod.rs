// solution.rs
//

// #[path = "1.两数之和.rs"]
// mod _0001_two_sum;
//
// #[path = "./10.正则表达式匹配.rs"]
// mod _0010_is_match;
//
// #[path = "./878.第-n-个神奇数字.rs"]
// mod _0878_is_match;

// #[path="./102.二叉树的层序遍历.rs"]
// mod _0102_bt_level_travel;
#![allow(dead_code)]

use std::cell::*;
use std::rc::*;

automod::dir!("src/solution");

pub struct Solution;

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

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}
