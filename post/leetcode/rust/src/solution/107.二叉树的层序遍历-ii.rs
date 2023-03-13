/*
 * @lc app=leetcode.cn id=107 lang=rust
 *
 * [107] 二叉树的层序遍历 II
 *
 * https://leetcode.cn/problems/binary-tree-level-order-traversal-ii/description/
 *
 * algorithms
 * Medium (72.24%)
 * Likes:    666
 * Dislikes: 0
 * Total Accepted:    259.7K
 * Total Submissions: 359K
 * Testcase Example:  '[3,9,20,null,null,15,7]'
 *
 * 给你二叉树的根节点 root ，返回其节点值 自底向上的层序遍历 。 （即按从叶子节点所在层到根节点所在的层，逐层从左向右遍历）
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：root = [3,9,20,null,null,15,7]
 * 输出：[[15,7],[9,20],[3]]
 *
 *
 * 示例 2：
 *
 *
 * 输入：root = [1]
 * 输出：[[1]]
 *
 *
 * 示例 3：
 *
 *
 * 输入：root = []
 * 输出：[]
 *
 *
 *
 *
 * 提示：
 *
 *
 * 树中节点数目在范围 [0, 2000] 内
 * -1000 <= Node.val <= 1000
 *
 *
 */

// @lc code=start
// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
impl Solution {
    /// ## 解题思路
    /// - 队列
    pub fn level_order_bottom(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut ret_vec = Vec::new();
        let mut q = VecDeque::new();
        if let Some(root) = root {
            q.push_back(root);
            while !q.is_empty() {
                let mut row_vals = Vec::new();
                let mut q_next = VecDeque::new();
                while let Some(node) = q.pop_front() {
                    row_vals.push(node.borrow().val);
                    if let Some(left) = &node.borrow().left {
                        q_next.push_back(left.clone());
                    }
                    if let Some(right) = &node.borrow().right {
                        q_next.push_back(right.clone());
                    }
                }

                ret_vec.push(row_vals);
                if !q_next.is_empty() {
                    q = q_next;
                }
            }
        }

        ret_vec.reverse();
        ret_vec
    }
}
// @lc code=end
//
use super::*;
