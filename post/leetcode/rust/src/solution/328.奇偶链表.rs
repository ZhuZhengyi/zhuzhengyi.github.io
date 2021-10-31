/*
 * @lc app=leetcode.cn id=328 lang=rust
 *
 * [328] 奇偶链表
 *
 * https://leetcode-cn.com/problems/odd-even-linked-list/description/
 *
 * algorithms
 * Medium (61.26%)
 * Likes:    133
 * Dislikes: 0
 * Total Accepted:    27.3K
 * Total Submissions: 44.5K
 * Testcase Example:  '[1,2,3,4,5]'
 *
 * 给定一个单链表，把所有的奇数节点和偶数节点分别排在一起。请注意，这里的奇数节点和偶数节点指的是节点编号的奇偶性，而不是节点的值的奇偶性。
 * 
 * 请尝试使用原地算法完成。你的算法的空间复杂度应为 O(1)，时间复杂度应为 O(nodes)，nodes 为节点总数。
 * 
 * 示例 1:
 * 
 * 输入: 1->2->3->4->5->NULL
 * 输出: 1->3->5->2->4->NULL
 * 
 * 
 * 示例 2:
 * 
 * 输入: 2->1->3->5->6->4->7->NULL 
 * 输出: 2->3->6->7->1->5->4->NULL
 * 
 * 说明:
 * 
 * 
 * 应当保持奇数节点和偶数节点的相对顺序。
 * 链表的第一个节点视为奇数节点，第二个节点视为偶数节点，以此类推。
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
    pub fn odd_even_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {

        let mut dummy_list1 = Some(Box::new(ListNode::new(0)));
        {
            let mut dummy_list2 = Some(Box::new(ListNode::new(0)));
            let mut tail_ptr1 = &mut dummy_list1;
            {
                let mut tail_ptr2 = &mut dummy_list2;

                let mut even = true;
                let mut head = head;
                while let Some(mut current_node) = head.take() {
                    head = current_node.next.take();
                    if even {
                        tail_ptr1.as_mut().unwrap().next = Some(current_node);
                        tail_ptr1 = &mut tail_ptr1.as_mut().unwrap().next;
                    } else {
                        tail_ptr2.as_mut().unwrap().next = Some(current_node);
                        tail_ptr2 = &mut tail_ptr2.as_mut().unwrap().next;
                    }
                    even = !even;
                }
            }

            if let Some(node2) = dummy_list2.as_mut().unwrap().next.take() {
                tail_ptr1.as_mut().unwrap().next = Some(node2);
            }
        }
        
        dummy_list1.unwrap().next
    }
}
// @lc code=end

