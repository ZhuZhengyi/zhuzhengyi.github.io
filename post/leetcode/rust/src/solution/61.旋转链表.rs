/*
 * @lc app=leetcode.cn id=61 lang=rust
 *
 * [61] 旋转链表
 *
 * https://leetcode-cn.com/problems/rotate-list/description/
 *
 * algorithms
 * Medium (39.96%)
 * Likes:    203
 * Dislikes: 0
 * Total Accepted:    44K
 * Total Submissions: 110.1K
 * Testcase Example:  '[1,2,3,4,5]\n2'
 *
 * 给定一个链表，旋转链表，将链表每个节点向右移动 k 个位置，其中 k 是非负数。
 *
 * 示例 1:
 *
 * 输入: 1->2->3->4->5->NULL, k = 2
 * 输出: 4->5->1->2->3->NULL
 * 解释:
 * 向右旋转 1 步: 5->1->2->3->4->NULL
 * 向右旋转 2 步: 4->5->1->2->3->NULL
 *
 *
 * 示例 2:
 *
 * 输入: 0->1->2->NULL, k = 4
 * 输出: 2->0->1->NULL
 * 解释:
 * 向右旋转 1 步: 2->0->1->NULL
 * 向右旋转 2 步: 1->2->0->NULL
 * 向右旋转 3 步: 0->1->2->NULL
 * 向右旋转 4 步: 2->0->1->NULL
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
    //获取单链表总节点数
    fn get_list_node_count(head: &Option<Box<ListNode>>) -> i32 {
        let mut ptr: Option<&Box<ListNode>> = head.as_ref();
        let mut node_count: i32 = 0;
        //遍历获取总结点数
        while let Some(node) = ptr {
            node_count += 1;
            ptr = node.next.as_ref();
        }

        node_count
    }

    //获取单链表最后节点引用
    fn get_list_last_node_ref(head: &mut Option<Box<ListNode>>) -> Option<&mut Box<ListNode>> {
        let mut ptr: Option<&mut Box<ListNode>> = head.as_mut();
        while let Some(node) = ptr {
            if node.next.is_none() {
                ptr = Some(node);
                break;
            }
            ptr = node.next.as_mut();
        }
        ptr
    }

    // 获取第n个节点的可变引用
    fn get_list_n_node_mut_ref(head: &mut Option<Box<ListNode>>, n: i32) -> &mut Box<ListNode> {
        let mut ptr: &mut Box<ListNode> = head.as_mut().unwrap();
        for _ in 0..n {
            ptr = ptr.next.as_mut().unwrap();
        }
        ptr
    }

    //
    pub fn rotate_right(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        //头结点为空或k为0
        if head.is_none() || k <= 0 {
            return head;
        }

        //获取总节点数
        let total_nodes = Solution::get_list_node_count(&head);

        //获取截断结点数
        let cut_nodes = total_nodes - k % total_nodes;
        if cut_nodes == total_nodes {
            return head;
        }

        let mut head = head;
        //获取截断点上一个结点可变借用
        let pre_cut_node_ptr: &mut Box<ListNode> =
            Solution::get_list_n_node_mut_ref(&mut head, cut_nodes - 1);

        //释放截断节点所有权，转移给new_head
        let mut new_head: Option<Box<ListNode>> = pre_cut_node_ptr.next.take();

        //获取截断链表的最后一个结点可变引用
        let last_node_ptr = Solution::get_list_last_node_ref(&mut new_head);

        //转移之前链表head所有权给新链表最后一个结点的next
        last_node_ptr.unwrap().next = head;

        new_head
    }

    /*
    pub fn rotate_right(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        //
        if head.is_none() || k <= 0 { return head }

        //遍历获取总节点数
        let mut ptr: Option<&Box<ListNode>> = head.as_ref();
        let mut node_count: i32 = 0;
        //遍历获取总结点数
        while let Some(node) = ptr {
            node_count += 1;
            ptr = node.next.as_ref();
        }

        //获取截断结点数
        let cut_nodes = node_count - k % node_count;
        if cut_nodes == node_count {
            return head;
        }

        //获取截断点上一个结点可变借用
        /*
                        ptr
                        |
        head: 1 -> 2 -> 3 -> 4 -> 5 -> NULL
        */
        let mut head = head;
        let mut ptr: &mut Box<ListNode> = head.as_mut().unwrap();
        for _ in 0..cut_nodes-1 {
            ptr = ptr.next.as_mut().unwrap();
        }

        //通过上一个节点释放并重新获取截断结点所有权
        /*
                        ptr
                        |
        head: 1 -> 2 -> 3
        new_head: 4 -> 5 -> NULL
        */
        let mut new_head: Option<Box<ListNode>> = ptr.next.take();


        /*
        head: 1 -> 2 -> 3
                  ptr
                  |
        new_head: 4 -> 5 -> NULL
        */
        //
        let mut ptr: Option<&mut Box<ListNode>>  = new_head.as_mut();
        /*
        head: 1 -> 2 -> 3
                       ptr
                       |
        new_head: 4 -> 5 -> NULL
        */
        //
        while let Some(node) = ptr {
            if node.next.is_none() {
                ptr = Some(node);
                break;
            }
            ptr = node.next.as_mut();
        }

        /*
                       ptr
                       |
        new_head: 4 -> 5 -> 1 -> 2 ->
        */
        //new_head最后节点的next获得head所有权
        ptr.unwrap().next = head;

        new_head

    }
    */
}
// @lc code=end
