/*
 * @lc app=leetcode.cn id=203 lang=rust
 *
 * [203] 移除链表元素
 *
 * https://leetcode.cn/problems/remove-linked-list-elements/description/
 *
 * algorithms
 * Easy (54.76%)
 * Likes:    1199
 * Dislikes: 0
 * Total Accepted:    524.2K
 * Total Submissions: 957.1K
 * Testcase Example:  '[1,2,6,3,4,5,6]\n6'
 *
 * 给你一个链表的头节点 head 和一个整数 val ，请你删除链表中所有满足 Node.val == val 的节点，并返回 新的头节点 。
 *
 *
 * 示例 1：
 *
 *
 * 输入：head = [1,2,6,3,4,5,6], val = 6
 * 输出：[1,2,3,4,5]
 *
 *
 * 示例 2：
 *
 *
 * 输入：head = [], val = 1
 * 输出：[]
 *
 *
 * 示例 3：
 *
 *
 * 输入：head = [7,7,7,7], val = 7
 * 输出：[]
 *
 *
 *
 *
 * 提示：
 *
 *
 * 列表中的节点数目在范围 [0, 10^4] 内
 * 1
 * 0
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
    /// - 增加一个空的链表头指针;
    pub fn remove_elements1(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        let mut pre_head = Box::new(ListNode::new(0));
        pre_head.next = head;
        let mut p = &mut pre_head;
        while let Some(node) = p.next.take() {
            if node.val == val {
                p.next = node.next;
            } else {
                p.next = Some(node);
                p = p.next.as_mut().unwrap();
            }
        }

        pre_head.next
    }

    /// 递归
    pub fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        match head {
            None => None,
            Some(mut head_node) => {
                let remaing = Self::remove_elements(head_node.next, val);
                if head_node.val == val {
                    remaing
                } else {
                    head_node.next = remaing;
                    Some(head_node)
                }
            }
        }
    }
}
// @lc code=end
