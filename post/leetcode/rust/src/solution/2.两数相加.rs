/*
 * @lc app=leetcode.cn id=2 lang=rust
 *
 * [2] 两数相加
 *
 * https://leetcode-cn.com/problems/add-two-numbers/description/
 *
 * algorithms
 * Medium (36.17%)
 * Likes:    3748
 * Dislikes: 0
 * Total Accepted:    301.1K
 * Total Submissions: 827.7K
 * Testcase Example:  '[2,4,3]\n[5,6,4]'
 *
 * 给出两个 非空 的链表用来表示两个非负的整数。其中，它们各自的位数是按照 逆序 的方式存储的，并且它们的每个节点只能存储 一位 数字。
 * 
 * 如果，我们将这两个数相加起来，则会返回一个新的链表来表示它们的和。
 * 
 * 您可以假设除了数字 0 之外，这两个数都不会以 0 开头。
 * 
 * 示例：
 * 
 * 输入：(2 -> 4 -> 3) + (5 -> 6 -> 4)
 * 输出：7 -> 0 -> 8
 * 原因：342 + 465 = 807
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
    /// ## 解题思路
    /// 1. 新建一个空链表d；
    /// 2. 依次从头取出l1,l2的各个节点，计算对应节点数字和；
    /// 3. 根据和及是否进位生产结果节点，append到链表d末尾；
    /// 4. 当l1,l2所有节点遍历完，且进位数也为0时，结束遍历；
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let (mut l1, mut l2) = (l1, l2);
        let mut dummy_head = Some(Box::new(ListNode::new(0)));
        let mut tail = &mut dummy_head;
        let (mut l1_end, mut l2_end, mut overflow) = (false, false, 0_i32);
        let result = loop {
            let val1 = match l1 {
                Some(node) => {
                    l1 = node.next;
                    node.val
                }
                None => {
                    l1_end = true;
                    0
                }
            };
            let val2 = match l2 {
                Some(node) => {
                    l2 = node.next;
                    node.val
                }
                None => {
                    l2_end = true;
                    0
                }
            };
            // 如果l1,l2都结束，且进位标志也为0，则跳出loop
            if l1_end && l2_end && overflow == 0 {
                break dummy_head.unwrap().next;
            }
            // 否则计算当前节点和进位值
            let sum = val1 + val2 + overflow;
            overflow = if sum > 9 { 1 } else { 0 };
            let sum = sum % 10 ;

            // 将sum节点append 新链表尾部
            tail.as_mut().unwrap().next = Some(Box::new(ListNode::new(sum)));
            // 移动尾指针
            tail = &mut tail.as_mut().unwrap().next;
        };
        
        result
    }
}
// @lc code=end

