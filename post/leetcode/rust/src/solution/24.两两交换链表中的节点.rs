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
    /// ## 解题思路
    /// - 指针交换
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode { val: 0, next: head }); //add dummy_head before head
        let mut p_ref = dummy.as_mut();

        // 如果后面存在两个节点
        while p_ref.next.is_some() 
            && p_ref.next.as_ref().unwrap().next.is_some() {
            //取下当前节点的下一个节点
            if let Some(mut first) = p_ref.next.take() {
                //取下下一个节点的下一个为second
                if let Some(mut second) = first.next.take() {
                    first.next = second.next.take(); //将第一个节点next指向second的下一个
                    second.next = Some(first); // 
                    p_ref.next = Some(second);

                    p_ref = p_ref.next.as_mut().unwrap();
                    p_ref = p_ref.next.as_mut().unwrap();
                }
            }
        }

        dummy.next
    }
}
// @lc code=end
