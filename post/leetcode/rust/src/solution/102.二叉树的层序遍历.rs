/*
 * @lc app=leetcode.cn id=102 lang=rust
 *
 * [102] 二叉树的层次遍历
 *
 * https://leetcode-cn.com/problems/binary-tree-level-order-traversal/description/
 *
 * algorithms
 * Medium (60.88%)
 * Likes:    391
 * Dislikes: 0
 * Total Accepted:    82.8K
 * Total Submissions: 135.8K
 * Testcase Example:  '[3,9,20,null,null,15,7]'
 *
 * 给定一个二叉树，返回其按层次遍历的节点值。 （即逐层地，从左到右访问所有节点）。
 *
 * 例如:
 * 给定二叉树: [3,9,20,null,null,15,7],
 *
 * ⁠   3
 * ⁠  / \
 * ⁠ 9  20
 * ⁠   /  \
 * ⁠  15   7
 *
 *
 * 返回其层次遍历结果：
 *
 * [
 * ⁠ [3],
 * ⁠ [9,20],
 * ⁠ [15,7]
 * ]
 *
 *
 */

use super::*;

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
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut res = Vec::new(); //result

        if let Some(root) = root {
            let mut cur_level_nodes = VecDeque::new();
            cur_level_nodes.push_back(root.clone());
            while !cur_level_nodes.is_empty() {
                let mut next_level_nodes = VecDeque::new();
                let mut cur_level_vals = Vec::new();
                while let Some(node) = cur_level_nodes.pop_front() {
                    cur_level_vals.push(node.borrow().val);
                    if let Some(left) = &node.borrow().left {
                        next_level_nodes.push_back(left.clone());
                    }
                    if let Some(right) = &node.borrow().right {
                        next_level_nodes.push_back(right.clone());
                    }
                }

                res.push(cur_level_vals);
                if !next_level_nodes.is_empty() {
                    cur_level_nodes = next_level_nodes;
                }
            }
        }

        res
    }
}
// @lc code=end

#[cfg(test)]
mod tests {

    #[test]
    fn test_level_order() {
        // assert_eq!(
        //     // Solution::level_order(build_tree(&vec![3, 9, 20, None, None, 15, 7])),
        //     // vec![vec![3], vec![9, 20], vec![15, 7],]
        // );
    }
}
