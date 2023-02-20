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
        if head.is_none() {
            return None;
        }

        let mut h = head;
        let mut p1 = h.as_mut().unwrap();
        while let Some(p2) = p1.next.as_mut() {
            if p2.val == p1.val {
                p1.next = p2.next.take();
            } else {
                p1 = p1.next.as_mut().unwrap();
            }
        }

        h
    }
}
// @lc code=end
