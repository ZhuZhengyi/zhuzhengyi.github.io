/*
 * @lc app=leetcode.cn id=24 lang=rust
 *
 * [24] 两两交换链表中的节点
 *
 * https://leetcode-cn.com/problems/swap-nodes-in-pairs/description/
 *
 * algorithms
 * Medium (63.29%)
 * Likes:    359
 * Dislikes: 0
 * Total Accepted:    58.3K
 * Total Submissions: 92.2K
 * Testcase Example:  '[1,2,3,4]'
 *
 * 给定一个链表，两两交换其中相邻的节点，并返回交换后的链表。
 * 
 * 你不能只是单纯的改变节点内部的值，而是需要实际的进行节点交换。
 * 
 * 
 * 
 * 示例:
 * 
 * 给定 1->2->3->4, 你应该返回 2->1->4->3.
 * 
 * 
 */

use super::*;

// @lc code=start
// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy_head = Some(Box::new(ListNode{ val: 0, next: head })); //add dummy_head before head
        let mut p = dummy_head.as_mut(); 

        //
        loop {
            //take first node away from head next
            let mut first = p.as_mut().unwrap().next.take();
            if first.is_none() {
                break;
            }
            //take second node away from first next node
            let mut second = first.as_mut().unwrap().next.take();
            if second.is_none() {
                p.as_mut().unwrap().next = first;  //
                break;
            }

            //swap first and second node
            // take second next node away frome second
            let mut second_next = second.as_mut().unwrap().next.take();
            // move second next node first next node
            first.as_mut().unwrap().next = second_next;
            // move first node to second next node
            second.as_mut().unwrap().next = first;
            // move second node to p next node
            p.as_mut().unwrap().next = second;

            // p travels 2 steps next
            p = p.unwrap().next.as_mut();
            p = p.unwrap().next.as_mut();
        }

        dummy_head.unwrap().next
    }
}
// @lc code=end

