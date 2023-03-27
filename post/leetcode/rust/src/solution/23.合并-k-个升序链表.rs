/*
 * @lc app=leetcode.cn id=23 lang=rust
 *
 * [23] 合并K个升序链表
 *
 * https://leetcode.cn/problems/merge-k-sorted-lists/description/
 *
 * algorithms
 * Hard (57.61%)
 * Likes:    2366
 * Dislikes: 0
 * Total Accepted:    615K
 * Total Submissions: 1.1M
 * Testcase Example:  '[[1,4,5],[1,3,4],[2,6]]'
 *
 * 给你一个链表数组，每个链表都已经按升序排列。
 * 
 * 请你将所有链表合并到一个升序链表中，返回合并后的链表。
 * 
 * 
 * 
 * 示例 1：
 * 
 * 输入：lists = [[1,4,5],[1,3,4],[2,6]]
 * 输出：[1,1,2,3,4,4,5,6]
 * 解释：链表数组如下：
 * [
 * ⁠ 1->4->5,
 * ⁠ 1->3->4,
 * ⁠ 2->6
 * ]
 * 将它们合并到一个有序链表中得到。
 * 1->1->2->3->4->4->5->6
 * 
 * 
 * 示例 2：
 * 
 * 输入：lists = []
 * 输出：[]
 * 
 * 
 * 示例 3：
 * 
 * 输入：lists = [[]]
 * 输出：[]
 * 
 * 
 * 
 * 
 * 提示：
 * 
 * 
 * k == lists.length
 * 0 <= k <= 10^4
 * 0 <= lists[i].length <= 500
 * -10^4 <= lists[i][j] <= 10^4
 * lists[i] 按 升序 排列
 * lists[i].length 的总和不超过 10^4
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
    /// - 递归
    /// 1. k == 0, 返回None;
    /// 2. k == 1, 返回lists[0];
    /// 3. k > 1, 返回将lists对半切开,分别合并后,再将两者合并;
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        fn merge_list(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>>{
            match (list1, list2) {
                (None, None) => None,
                (None, Some(list2)) => Some(list2),
                (Some(list1), None) => Some(list1),
                (Some(mut list1), Some(mut list2)) => {
                    if list1.val < list2.val {
                        list1.next = merge_list(list1.next.take(), Some(list2));
                        Some(list1)
                    } else {
                        list2.next = merge_list(Some(list1), list2.next.take());
                        Some(list2)
                    }
                }
            }
        }
        match lists.len() {
            0 => None,
            1 => lists[0].clone(),
            l => {
                merge_list(Self::merge_k_lists(lists[0..l/2].to_vec()), Self::merge_k_lists(lists[l/2..].to_vec()))
            }
        }
    }
}
// @lc code=end

