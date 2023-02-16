/*
 * @lc app=leetcode.cn id=82 lang=rust
 *
 * [82] 删除排序链表中的重复元素 II
 *
 * https://leetcode-cn.com/problems/remove-duplicates-from-sorted-list-ii/description/
 *
 * algorithms
 * Medium (45.93%)
 * Likes:    221
 * Dislikes: 0
 * Total Accepted:    33.1K
 * Total Submissions: 71.9K
 * Testcase Example:  '[1,2,3,3,4,4,5]'
 *
 * 给定一个排序链表，删除所有含有重复数字的节点，只保留原始链表中 没有重复出现 的数字。
 * 
 * 示例 1:
 * 
 * 输入: 1->2->3->3->4->4->5
 * 输出: 1->2->5
 * 
 * 
 * 示例 2:
 * 
 * 输入: 1->1->1->2->3
 * 输出: 2->3
 * 
 */

use super::*;
pub struct Solution;

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
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy_head = Some(Box::new(ListNode::new(0)));
        let mut ptr = &mut dummy_head;

        let mut pre_val: i32 = -1;
        let mut pre_val_init = false;
        let mut next_val: i32 = 0;
        let mut head = head;
        while let Some(mut node) = head {
            head = node.next.take();
            if head.is_some() {
                if node.val == head.as_ref().unwrap().val {
                    pre_val_init = true;
                    pre_val = node.val;
                    continue
                }
            }
            if pre_val_init && pre_val == node.val {
                continue
            }
            pre_val = node.val;
            pre_val_init = true;
            ptr.as_mut().unwrap().next = Some(node);
            ptr = &mut ptr.as_mut().unwrap().next;
        }

        dummy_head.unwrap().next

    }
}
// @lc code=end

