/*
 * @lc app=leetcode.cn id=83 lang=rust
 *
 * [83] 删除排序链表中的重复元素
 *
 * https://leetcode-cn.com/problems/remove-duplicates-from-sorted-list/description/
 *
 * algorithms
 * Easy (49.10%)
 * Likes:    260
 * Dislikes: 0
 * Total Accepted:    74.1K
 * Total Submissions: 150.7K
 * Testcase Example:  '[1,1,2]'
 *
 * 给定一个排序链表，删除所有重复的元素，使得每个元素只出现一次。
 * 
 * 示例 1:
 * 
 * 输入: 1->1->2
 * 输出: 1->2
 * 
 * 
 * 示例 2:
 * 
 * 输入: 1->1->2->3->3
 * 输出: 1->2->3
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
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy_head = Some(Box::new(ListNode::new(0)));
        let mut ptr = &mut dummy_head;

        let mut pre_val: i32 = -1;
        let mut pre_val_init = false;
        let mut head = head;
        while let Some(mut node) = head.take() {
            head = node.next.take();
            if pre_val_init && pre_val == node.val {
                continue
            }
            pre_val = node.val;
            pre_val_init = true;
            //
            ptr.as_mut().unwrap().next = Some(node);
            ptr = &mut ptr.as_mut().unwrap().next;
        }

        dummy_head.unwrap().next

    }
}
// @lc code=end

