/*
 * @lc app=leetcode.cn id=124 lang=rust
 *
 * [124] 二叉树中的最大路径和
 *
 * https://leetcode-cn.com/problems/binary-tree-maximum-path-sum/description/
 *
 * algorithms
 * Hard (44.52%)
 * Likes:    1264
 * Dislikes: 0
 * Total Accepted:    165.9K
 * Total Submissions: 372.7K
 * Testcase Example:  '[1,2,3]'
 *
 * 路径 被定义为一条从树中任意节点出发，沿父节点-子节点连接，达到任意节点的序列。同一个节点在一条路径序列中 至多出现一次 。该路径 至少包含一个
 * 节点，且不一定经过根节点。
 *
 * 路径和 是路径中各节点值的总和。
 *
 * 给你一个二叉树的根节点 root ，返回其 最大路径和 。
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：root = [1,2,3]
 * 输出：6
 * 解释：最优路径是 2 -> 1 -> 3 ，路径和为 2 + 1 + 3 = 6
 *
 * 示例 2：
 *
 *
 * 输入：root = [-10,9,20,null,null,15,7]
 * 输出：42
 * 解释：最优路径是 15 -> 20 -> 7 ，路径和为 15 + 20 + 7 = 42
 *
 *
 *
 *
 * 提示：
 *
 *
 * 树中节点数目范围是 [1, 3 * 10^4]
 * -1000
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
use std::cmp;
use std::rc::Rc;
impl Solution {
    /// ## 解题思路：
    /// 1. 树总最大路径和为经过每一个树节点的所有最大路径和中最大的:
    ///       max_path_sum = max(max_path_sum, f(node))
    /// 2. 经过每个节点的路径和:
    ///       f(node) = g(node.left) + node.val + g(node.right)，
    ///    其中：g(node.left): 表示以node.left为端点的一侧最大路径和；
    /// 3. g(node) = node.val + max(g(node.left), g(node.right))
    pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        // 以node为端点的最大一边路径和。
        fn max_path_sum_end_with_node(
            node: &Option<Rc<RefCell<TreeNode>>>,
            max_sum: &mut i32, //helper for max_path_sum
        ) -> i32 {
            match node {
                None => 0,
                Some(node) => {
                    let left_sum =
                        cmp::max(0, max_path_sum_end_with_node(&node.borrow().left, max_sum));
                    let right_sum =
                        cmp::max(0, max_path_sum_end_with_node(&node.borrow().right, max_sum));
                    *max_sum = cmp::max(*max_sum, left_sum + node.borrow().val + right_sum);
                    //
                    node.borrow().val + cmp::max(left_sum, right_sum)
                }
            }
        }

        let mut max_path = f32::NEG_INFINITY as i32;
        max_path_sum_end_with_node(&root, &mut max_path);
        max_path
    }
}
// @lc code=end
