/*
 * @lc app=leetcode.cn id=143 lang=rust
 *
 * [143] 重排链表
 *
 * https://leetcode-cn.com/problems/reorder-list/description/
 *
 * algorithms
 * Medium (54.54%)
 * Likes:    158
 * Dislikes: 0
 * Total Accepted:    16.8K
 * Total Submissions: 30.7K
 * Testcase Example:  '[1,2,3,4]'
 *
 * 给定一个单链表 L：L0→L1→…→Ln-1→Ln ，
 * 将其重新排列后变为： L0→Ln→L1→Ln-1→L2→Ln-2→…
 *
 * 你不能只是单纯的改变节点内部的值，而是需要实际的进行节点交换。
 *
 * 示例 1:
 *
 * 给定链表 1->2->3->4, 重新排列为 1->4->2->3.
 *
 * 示例 2:
 *
 * 给定链表 1->2->3->4->5, 重新排列为 1->5->2->4->3.
 *
 */

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
    /// 解题思路：
    /// 1. 
    pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
        if head.is_none() {
            return
        }

        //get list length
        let mut len = 0;
        {
            let mut ptr = head.as_ref();
            while let Some(node) = ptr {
                len += 1;
                ptr = node.next.as_ref();
            }
        }

        //if list length < 2,
        if len < 2 {
            return
        }

        //split list into 2 sub lists: pre_half with normal order and last_half with reserv order
        let mut sub_order_list = Some(Box::new(ListNode::new(0)));
        let mut sub_reverse_list = Some(Box::new(ListNode::new(0)));

        //first node left as head
        let mut next = head.as_mut().unwrap().next.take();
        {
            let mut i = 0;
            let cut_len = len / 2;
            let mut tail_ptr1 = &mut sub_order_list;
            while let Some(mut current_node) = next.take() {
                i += 1;
                next = current_node.next.take();
                if i > cut_len {
                    current_node.next = sub_revor_list.as_mut().unwrap().next.take();
                    sub_revor_list.as_mut().unwrap().next = Some(current_node);
                } else {
                    tail_ptr1.as_mut().unwrap().next = Some(current_node);
                    tail_ptr1 = &mut tail_ptr1.as_mut().unwrap().next;
                }
            }
        }

        // join sub_reverse_list with sub_order_list
        {
            let mut list1_tail_ptr = &mut sub_order_list;
            let mut list2_next = sub_reverse_list.as_mut().unwrap().next.take();
            while let Some(mut node) = list2_next {
                list2_next = node.next.take();
                node.next = list1_tail_ptr.as_mut().unwrap().next.take();
                list1_tail_ptr.as_mut().unwrap().next = Some(node);

                list1_tail_ptr = &mut list1_tail_ptr.as_mut().unwrap().next;
                list1_tail_ptr = &mut list1_tail_ptr.as_mut().unwrap().next;
            }
        }

        // join sub_order_list to left_head node
        head.as_mut().unwrap().next = sub_order_list.as_mut().unwrap().next.take();
    }
}
// @lc code=end

