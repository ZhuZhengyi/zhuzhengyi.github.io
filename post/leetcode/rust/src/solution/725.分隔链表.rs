/*
 * @lc app=leetcode.cn id=725 lang=rust
 *
 * [725] 分隔链表
 *
 * https://leetcode-cn.com/problems/split-linked-list-in-parts/description/
 *
 * algorithms
 * Medium (52.83%)
 * Likes:    43
 * Dislikes: 0
 * Total Accepted:    6.1K
 * Total Submissions: 11.4K
 * Testcase Example:  '[1,2,3,4]\n5'
 *
 * 给定一个头结点为 root 的链表, 编写一个函数以将链表分隔为 k 个连续的部分。
 *
 * 每部分的长度应该尽可能的相等: 任意两部分的长度差距不能超过 1，也就是说可能有些部分为 null。
 *
 * 这k个部分应该按照在链表中出现的顺序进行输出，并且排在前面的部分的长度应该大于或等于后面的长度。
 *
 * 返回一个符合上述规则的链表的列表。
 *
 * 举例： 1->2->3->4, k = 5 // 5 结果 [ [1], [2], [3], [4], null ]
 *
 * 示例 1：
 *
 *
 * 输入:
 * root = [1, 2, 3], k = 5
 * 输出: [[1],[2],[3],[],[]]
 * 解释:
 * 输入输出各部分都应该是链表，而不是数组。
 * 例如, 输入的结点 root 的 val= 1, root.next.val = 2, \root.next.next.val = 3, 且
 * root.next.next.next = null。
 * 第一个输出 output[0] 是 output[0].val = 1, output[0].next = null。
 * 最后一个元素 output[4] 为 null, 它代表了最后一个部分为空链表。
 *
 *
 * 示例 2：
 *
 *
 * 输入:
 * root = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10], k = 3
 * 输出: [[1, 2, 3, 4], [5, 6, 7], [8, 9, 10]]
 * 解释:
 * 输入被分成了几个连续的部分，并且每部分的长度相差不超过1.前面部分的长度大于等于后面部分的长度。
 *
 *
 *
 *
 * 提示:
 *
 *
 * root 的长度范围： [0, 1000].
 * 输入的每个节点的大小范围：[0, 999].
 * k 的取值范围： [1, 50].
 *
 *
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
    /// - 分割链表
    pub fn split_list_to_parts(root: Option<Box<ListNode>>, k: i32) -> Vec<Option<Box<ListNode>>> {
        let mut l = 0;
        let mut node = &root;
        while let Some(n) = node {
            l += 1;
            node = &n.next;
        }

        let n = l / k;
        let mut head = root;
        let mut res = Vec::with_capacity(k as usize);
        for i in 0..k as usize {
            res.push(head);
            let mut node = &mut res[i];
            for _ in 0..n + if i < (l % k) as usize { 1 } else { 0 } {
                if let Some(x) = node {
                    node = &mut x.next;
                }
            }
            head = node.take();
        }

        res
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    //use super::*;

    #[test]
    fn test() {}
}
