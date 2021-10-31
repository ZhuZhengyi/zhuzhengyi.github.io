/*
 * @lc app=leetcode.cn id=92 lang=rust
 *
 * [92] 反转链表 II
 *
 * https://leetcode-cn.com/problems/reverse-linked-list-ii/description/
 *
 * algorithms
 * Medium (49.10%)
 * Likes:    304
 * Dislikes: 0
 * Total Accepted:    34.8K
 * Total Submissions: 70.9K
 * Testcase Example:  '[1,2,3,4,5]\n2\n4'
 *
 * 反转从位置 m 到 n 的链表。请使用一趟扫描完成反转。
 * 
 * 说明:
 * 1 ≤ m ≤ n ≤ 链表长度。
 * 
 * 示例:
 * 
 * 输入: 1->2->3->4->5->NULL, m = 2, n = 4
 * 输出: 1->4->3->2->5->NULL
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
    pub fn reverse_between(head: Option<Box<ListNode>>, m: i32, n: i32) -> Option<Box<ListNode>> {
        if head.is_none() || n <= m  { 
            return head; 
        }

        let mut dummy_list: Option<Box<ListNode>> = Some(Box::new(ListNode::new(0)));
        let mut tail_ptr = &mut dummy_list;
        let mut tmp_list: Option<Box<ListNode>> = Some(Box::new(ListNode::new(0)));
        let mut i = 0;
        let mut next = head;
        while let Some(mut cur_node) = next.take() {
            i += 1;
            //取下当前节点下一个结点，将所有权转交给head
            next = cur_node.next.take(); 
            if i < m {
                tail_ptr.as_mut().unwrap().next = Some(cur_node);
                tail_ptr = &mut tail_ptr.as_mut().unwrap().next;
            } else if i >= m && i <= n {
                cur_node.next = tmp_list.as_mut().unwrap().next.take();
                tmp_list.as_mut().unwrap().next = Some(cur_node);
                if i == n {
                    //contact rev_head.next to tail
                    let mut tmp_node = tmp_list.as_mut().unwrap().next.take();
                    while let Some(mut node) = tmp_node {
                        tmp_node = node.next.take();
                        tail_ptr.as_mut().unwrap().next = Some(node);
                        tail_ptr = &mut tail_ptr.as_mut().unwrap().next;
                    }
                    //如果下一个结点不为空，
                    if next.is_some() {
                        tail_ptr.as_mut().unwrap().next = next;
                    }
                    break;
                }
            } else {
                break;
            }
        }

        dummy_list.unwrap().next
    }
}
// @lc code=end

