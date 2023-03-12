/*
 * @lc app=leetcode.cn id=257 lang=rust
 *
 * [257] 二叉树的所有路径
 *
 * https://leetcode-cn.com/problems/binary-tree-paths/description/
 *
 * algorithms
 * Easy (68.35%)
 * Likes:    596
 * Dislikes: 0
 * Total Accepted:    148.3K
 * Total Submissions: 216.7K
 * Testcase Example:  '[1,2,3,null,5]'
 *
 * 给你一个二叉树的根节点 root ，按 任意顺序 ，返回所有从根节点到叶子节点的路径。
 * 
 * 叶子节点 是指没有子节点的节点。
 * 
 * 
 * 示例 1：
 * 
 * 
 * 输入：root = [1,2,3,null,5]
 * 输出：["1->2->5","1->3"]
 * 
 * 
 * 示例 2：
 * 
 * 
 * 输入：root = [1]
 * 输出：["1"]
 * 
 * 
 * 
 * 
 * 提示：
 * 
 * 
 * 树中节点的数目在范围 [1, 100] 内
 * -100 <= Node.val <= 100
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
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    /// ## 解题思路：
    /// - 迭代
    /// 1. 中序递归遍历二叉树每个节点, 遍历时，记录每次遍历的路径
    /// 2. 到达叶子节点，则将路径加入到结果集中；
    /// 3. 遍历时，注意出入栈操作；
    pub fn binary_tree_paths(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<String> {

        /// helper
        fn helper(node: &Option<Rc<RefCell<TreeNode>>>, node_path: &mut Vec<i32>, res: &mut Vec<String>)  {
            match node {
                None => {}
                Some(node) if node.borrow().left.is_none() && node.borrow().right.is_none() => {
                    node_path.push(node.borrow().val);
                    res.push(node_path.iter().clone().map(|x| x.to_string()).collect::<Vec<_>>().join("->"));
                    node_path.pop();
                }
                Some(node) => {
                    node_path.push(node.borrow().val);
                    helper(&node.borrow().left, node_path, res);
                    helper(&node.borrow().right, node_path, res);
                    node_path.pop();
                }
            }
        }

        let mut node_path: Vec<i32> = vec![];
        let mut res: Vec<String> = vec![];

        helper(&root, &mut node_path, &mut res);

        res
    }
}
// @lc code=end

