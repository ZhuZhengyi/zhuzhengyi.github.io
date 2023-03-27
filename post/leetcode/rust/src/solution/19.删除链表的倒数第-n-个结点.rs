/*
 * @lc app=leetcode.cn id=19 lang=rust
 *
 * [19] 删除链表的倒数第 N 个结点
 *
 * https://leetcode.cn/problems/remove-nth-node-from-end-of-list/description/
 *
 * algorithms
 * Medium (45.32%)
 * Likes:    2477
 * Dislikes: 0
 * Total Accepted:    1.1M
 * Total Submissions: 2.4M
 * Testcase Example:  '[1,2,3,4,5]\n2'
 *
 * 给你一个链表，删除链表的倒数第 n 个结点，并且返回链表的头结点。
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：head = [1,2,3,4,5], n = 2
 * 输出：[1,2,3,5]
 *
 *
 * 示例 2：
 *
 *
 * 输入：head = [1], n = 1
 * 输出：[]
 *
 *
 * 示例 3：
 *
 *
 * 输入：head = [1,2], n = 1
 * 输出：[1]
 *
 *
 *
 *
 * 提示：
 *
 *
 * 链表中结点的数目为 sz
 * 1 <= sz <= 30
 * 0 <= Node.val <= 100
 * 1 <= n <= sz
 *
 *
 *
 *
 * 进阶：你能尝试使用一趟扫描实现吗？
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
    /// - 双指针 + dummy node
    /// 1. 在原表头前面增加一个dummy, 以统一处理原head结点需要处理的情况;
    /// 2. 设置2个指针, 一前一后遍历链表, 前指针提前n个节点;
    /// 3. 然后将前后指针同时遍历,直到前指针到达尾节点;
    /// 4. 将前指针next指向其next->next;
    pub fn remove_nth_from_end(mut head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        // 设置dummy节点
        let mut dummy = Box::new(ListNode::new(0));
        dummy.next = head;
        let mut pre_p = dummy.clone(); //
        for _ in 0..(n as usize) {
            pre_p = pre_p.next.unwrap();
        }
        let mut p = dummy.as_mut();
        while pre_p.next.is_some() {
            pre_p = pre_p.next.unwrap();
            p = p.next.as_mut().unwrap();
        }
        p.next = p.next.as_mut().unwrap().next.take();

        dummy.next
    }
}
// @lc code=end
