// 
// @lc app=leetcode.cn id=21 lang=rust

// @lc code=start
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//    pub val: i32,
//    pub next: Option<Box<ListNode>>,
// }
//impl ListNode {
 //   #[inline]
 //  fn new(val: i32) -> Self {
 //       ListNode {
//            next: None,
//            val
//        }
//    }
//}
impl Solution {
    /// ## 解题思路
    /// * 同时递归遍历两个链表；
    /// * 遍历时, 比较两个链表结点值;
    ///   - 如果两个节点都为空，则为空；
    ///   - 如果有一个为空，则返回非空节点；
    ///   - 如果都不为空，则将小的节点取下，将剩下的和另一条链表递归合并；
    pub fn merge_two_lists(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match (l1, l2) {
            (None, None) => None,
            (None, r) => r,
            (l, None) => l,
            (Some(mut l), Some(mut r)) => {
                if (l.val < r.val) {
                    l.next = Self::merge_two_lists(l.next, Some(r));
                    Some(l)
                } else {
                    r.next = Self::merge_two_lists(Some(l), r.next);
                    Some(r)
                }
            }
        }
    }
}
// @lc code=end