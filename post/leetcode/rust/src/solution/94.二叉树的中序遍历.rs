/*
 * @lc app=leetcode.cn id=94 lang=rust
 *
 * [94] 二叉树的中序遍历
 *
 * https://leetcode-cn.com/problems/binary-tree-inorder-traversal/description/
 *
 * algorithms
 * Medium (70.23%)
 * Likes:    406
 * Dislikes: 0
 * Total Accepted:    109.4K
 * Total Submissions: 155.6K
 * Testcase Example:  '[1,null,2,3]'
 *
 * 给定一个二叉树，返回它的中序 遍历。
 *
 * 示例:
 *
 * 输入: [1,null,2,3]
 * ⁠  1
 * ⁠   \
 * ⁠    2
 * ⁠   /
 * ⁠  3
 *
 * 输出: [1,3,2]
 *
 * 进阶: 递归算法很简单，你可以通过迭代算法完成吗？
 *
 */

use super::*;
struct Solution;

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
    /// - 递归+深度遍历
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        fn inorder_dfs(node: &Option<Rc<RefCell<TreeNode>>>, res: &mut Vec<i32>) {
            match node {
                None => {}
                Some(x) => {
                    inorder_dfs(&x.borrow().left, res);
                    res.push(x.borrow().val);
                    inorder_dfs(&x.borrow().right, res);
                }
            }
        }

        let mut res = vec![];
        inorder_dfs(&root, &mut res);

        res
    }
}
// @lc code=end
