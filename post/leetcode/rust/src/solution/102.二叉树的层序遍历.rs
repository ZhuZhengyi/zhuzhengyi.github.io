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
use std::rc::Rc;
impl Solution {
    /// ## 解题思路
    /// - 队列
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut res = Vec::new(); //result

        if root.is_none() {
            return res;
        }

        let mut level = Vec::new();
        level.push(root.unwrap().clone());

        let mut nodes_levels = Vec::new();
        nodes_levels.push(level);
        while !nodes_levels.is_empty() {
            let level = nodes_levels.remove(0); //取出一层
            let mut next_level = Vec::new(); //下层节点
            let mut level_res = Vec::new(); //保存每一层的结果

            // 依次取出当前层下层子树节点，放入下层队列中
            level.iter().for_each(|node| {
                // 将本层节点数据加入到结果中
                level_res.push(node.borrow().val);
                if let Some(l) = node.borrow().left.clone() {
                    next_level.push(l);
                }
                if let Some(r) = node.borrow().right.clone() {
                    next_level.push(r);
                }
            });

            // 将下层队列加入到队列末尾
            if !next_level.is_empty() {
                nodes_levels.push(next_level);
            }

            // 将本层
            res.push(level_res);
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
