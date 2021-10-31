/*
 * @lc app=leetcode.cn id=86 lang=rust
 *
 * [86] 分隔链表
 *
 * https://leetcode-cn.com/problems/partition-list/description/
 *
 * algorithms
 * Medium (55.90%)
 * Likes:    158
 * Dislikes: 0
 * Total Accepted:    26.4K
 * Total Submissions: 47.2K
 * Testcase Example:  '[1,4,3,2,5,2]\n3'
 *
 * 给定一个链表和一个特定值 x，对链表进行分隔，使得所有小于 x 的节点都在大于或等于 x 的节点之前。
 * 
 * 你应当保留两个分区中每个节点的初始相对位置。
 * 
 * 示例:
 * 
 * 输入: head = 1->4->3->2->5->2, x = 3
 * 输出: 1->2->2->4->3->5
 * 
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
    pub fn partition(head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
        if head.is_none() {
            return head
        }

        let mut dummy_head1 = Some(Box::new(ListNode::new(0)));
        let mut dummy_head2 = Some(Box::new(ListNode::new(0)));
        let mut ptr1 = &mut dummy_head1;
        let mut ptr2 = &mut dummy_head2;

        let mut head = head;
        while let Some(mut node) = head {
            head = node.next.take();
            if node.val < x {
                ptr1.as_mut().unwrap().next = Some(node);
                ptr1 = &mut ptr1.as_mut().unwrap().next;
            } else {
                ptr2.as_mut().unwrap().next = Some(node);
                ptr2 = &mut ptr2.as_mut().unwrap().next;
            }
        }

        if let Some(mut node) = dummy_head2.as_mut().unwrap().next.take() {
            ptr1.as_mut().unwrap().next = Some(node);
        }

        dummy_head1.unwrap().next
    }
}
// @lc code=end

