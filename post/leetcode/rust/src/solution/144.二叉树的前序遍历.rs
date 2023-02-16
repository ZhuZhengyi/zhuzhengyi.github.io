/*
 * @lc app=leetcode.cn id=144 lang=rust
 *
 * [144] 二叉树的前序遍历
 *
 * https://leetcode-cn.com/problems/binary-tree-preorder-traversal/description/
 *
 * algorithms
 * Easy (70.42%)
 * Likes:    666
 * Dislikes: 0
 * Total Accepted:    428.8K
 * Total Submissions: 608.9K
 * Testcase Example:  '[1,null,2,3]'
 *
 * 给你二叉树的根节点 root ，返回它节点值的 前序 遍历。
 * 
 * 
 * 
 * 示例 1：
 * 
 * 
 * 输入：root = [1,null,2,3]
 * 输出：[1,2,3]
 * 
 * 
 * 示例 2：
 * 
 * 
 * 输入：root = []
 * 输出：[]
 * 
 * 
 * 示例 3：
 * 
 * 
 * 输入：root = [1]
 * 输出：[1]
 * 
 * 
 * 示例 4：
 * 
 * 
 * 输入：root = [1,2]
 * 输出：[1,2]
 * 
 * 
 * 示例 5：
 * 
 * 
 * 输入：root = [1,null,2]
 * 输出：[1,2]
 * 
 * 
 * 
 * 
 * 提示：
 * 
 * 
 * 树中节点数目在范围 [0, 100] 内
 * -100 
 * 
 * 
 * 
 * 
 * 进阶：递归算法很简单，你可以通过迭代算法完成吗？
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
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    /// ## 解题思路
    /// * 前序遍历顺序： 根节点 -> 左子树 -> 右子树 
    /// * 两种方法
    pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        //Solution::preorder_traversal_iter(root)
        Solution::preorder_traversal_rec(&root)
    }

    /// 迭代
    /// 使用stack
    pub fn preorder_traversal_iter(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res = vec![];
        let mut stack = vec![];

        let mut node = root;
        loop {
            //先遍历每一个节点的左子树，并将节点放入栈中
            while let Some(n) = node {
                res.push(n.borrow().val); //现将当前节点输出
                stack.push(n.clone());    //
                node = n.borrow().left.clone();
            }

            //栈为空，则无待处理的节点，退出迭代
            if stack.is_empty() {
                break;
            }

            //左子树处理完，处理栈中节点
            node = stack.pop().unwrap().borrow().right.clone();
        }

        res
    }

    /// 递归
    pub fn preorder_traversal_rec(root: &Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        fn preorder_dfs(root: &Option<Rc<RefCell<TreeNode>>>, res: &mut Vec<i32>) {
            if let Some(n) = root {
                res.push(n.borrow().val);
                preorder_dfs(&n.borrow().left, res);
                preorder_dfs(&n.borrow().right, res);
            }
        }

        let mut res = Vec::new();
        preorder_dfs(root, &mut res);
        res
    }
}
// @lc code=end

