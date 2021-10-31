/*
 * @lc app=leetcode.cn id=206 lang=rust
 *
 * [206] 反转链表
 *
 * https://leetcode-cn.com/problems/reverse-linked-list/description/
 *
 * algorithms
 * Easy (65.94%)
 * Likes:    758
 * Dislikes: 0
 * Total Accepted:    152.1K
 * Total Submissions: 227.6K
 * Testcase Example:  '[1,2,3,4,5]'
 *
 * 反转一个单链表。
 * 
 * 示例:
 * 
 * 输入: 1->2->3->4->5->NULL
 * 输出: 5->4->3->2->1->NULL
 * 
 * 进阶:
 * 你可以迭代或递归地反转链表。你能否用两种方法解决这道题？
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
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut new_head = None;
        //循环取出原链表的头结点，加入到新链表的前面
        while let Some(mut node) = head {
            //take node next as head
            //   head: 3->4->NULL
            //   node: 2
            //   new_head:  1->NULL
            head = node.next.take();
            //  head: 3->4->
            //  node:  2->1->NULL
            //  new_head
            node.next = new_head;
            //  head:  3->4->NULL
            //  new_head: 2->1->NULL
            new_head = Some(node);
        }
        new_head
    }
}
// @lc code=end

