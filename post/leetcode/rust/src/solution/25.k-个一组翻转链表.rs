/*
 * @lc app=leetcode.cn id=25 lang=rust
 *
 * [25] K 个一组翻转链表
 *
 * https://leetcode-cn.com/problems/reverse-nodes-in-k-group/description/
 *
 * algorithms
 * Hard (55.23%)
 * Likes:    367
 * Dislikes: 0
 * Total Accepted:    36.4K
 * Total Submissions: 65K
 * Testcase Example:  '[1,2,3,4,5]\n2'
 *
 * 给你一个链表，每 k 个节点一组进行翻转，请你返回翻转后的链表。
 * 
 * k 是一个正整数，它的值小于或等于链表的长度。
 * 
 * 如果节点总数不是 k 的整数倍，那么请将最后剩余的节点保持原有顺序。
 * 
 * 示例 :
 * 
 * 给定这个链表：1->2->3->4->5
 * 
 * 当 k = 2 时，应当返回: 2->1->4->3->5
 * 
 * 当 k = 3 时，应当返回: 3->2->1->4->5
 * 
 * 说明 :
 * 
 * 
 * 你的算法只能使用常数的额外空间。
 * 你不能只是单纯的改变节点内部的值，而是需要实际的进行节点交换。
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
    pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        if k <= 1 || head.is_none(){
            return head;
        }

        let mut dummy_head = ListNode::new(0);
        let mut prev_next = &mut dummy_head.next;
        let mut tmp_nodes = vec![];
        let mut head = head;
        while let Some(mut node) = head.take() {
            head = node.next.take();
            tmp_nodes.push(node);

            if tmp_nodes.len() == k as usize {
                /*
                tmp_nodes.iter().rev().for_each(|tmp_node|{ 
                    *prev_next = Some(*tmp_node); 
                    prev_next = &mut prev_next.as_mut().unwrap().next;
                });
                */
                while tmp_nodes.len() > 0 {
                    *prev_next = tmp_nodes.pop();
                    prev_next = &mut prev_next.as_mut().unwrap().next;
                }
            } 
        }
        while tmp_nodes.len() > 0 {
            *prev_next = Some(tmp_nodes.remove(0));
            prev_next = &mut prev_next.as_mut().unwrap().next;
        }

        *prev_next = None;

        dummy_head.next
    }
}
// @lc code=end

