// solution/mod.rs
//

#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused_doc_comments)]
#![allow(unused_variables)]

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
            right: None,
        }
    }
}

// Definition for singly-linked list.
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

    pub fn get_list_node_count(head: &Option<Box<ListNode>>) -> i32 {
        let mut ptr = head.as_ref();
        let mut node_count: i32 = 0;
        //遍历获取总结点数
        while let Some(node) = ptr {
            node_count += 1;
            ptr = node.next.as_ref();
        }

        node_count
    }

    //获取单链表最后节点引用
    pub fn get_list_last_node_ref(head: &mut Option<Box<ListNode>>) -> Option<&mut Box<ListNode>> {
        let mut ptr: Option<&mut Box<ListNode>> = head.as_mut();
        while let Some(node) = ptr {
            if node.next.is_none() {
                ptr = Some(node);
                break;
            }
            ptr = node.next.as_mut();
        }
        ptr
    }
}
