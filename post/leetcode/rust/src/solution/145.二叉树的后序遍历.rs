/*
 * @lc app=leetcode.cn id=145 lang=rust
 *
 * [145] 二叉树的后序遍历
 *
 * https://leetcode-cn.com/problems/binary-tree-postorder-traversal/description/
 *
 * algorithms
 * Easy (75.09%)
 * Likes:    709
 * Dislikes: 0
 * Total Accepted:    323.2K
 * Total Submissions: 430K
 * Testcase Example:  '[1,null,2,3]'
 *
 * 给定一个二叉树，返回它的 后序 遍历。
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
 * 输出: [3,2,1]
 * 
 * 进阶: 递归算法很简单，你可以通过迭代算法完成吗？
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
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    /// ## 解题思路
    /// * 后序遍历的顺序： 左子树 -> 右子树 -> 根节点
    /// * 迭代： 
    pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        //Solution::postorder_traversal_rec(&root)
        Solution::postorder_traversal_iter(root)
    }

    /// 迭代
    pub fn postorder_traversal_iter(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res = vec![];
        let mut pending_nodes = vec![];

        let mut visited = None;
        let mut node = root;
        loop {
            //先深度遍历节点的左子树，将遍历过的节点放入栈中
            while let Some(n) = node {
                pending_nodes.push(n.clone());
                node = n.borrow().left.clone();
            }
            //栈为空，则无待处理的节点，退出迭代
            if pending_nodes.is_empty() {
                break;
            }
            // 检查栈顶元素
            //
            let r = pending_nodes.last().unwrap().borrow().right.clone();
            if r != visited {
                node = r;
                visited = None
            } else { //右子树已被遍历过
                if let Some(n) = pending_nodes.pop(){
                    res.push(n.borrow().val);
                    visited = Some(n.clone());
                }
            }
        }

        res
    }

    /// ## 解题思路
    /// * 后序遍历的顺序： 左子树 -> 右子树 -> 根节点
    /// * 递归： 
    pub fn postorder_traversal_rec(root: &Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        fn traversal(root: &Option<Rc<RefCell<TreeNode>>>, res: &mut Vec<i32>) {
            if let Some(n) = root {
                traversal(&n.borrow().left, res);
                traversal(&n.borrow().right, res);
                res.push(n.borrow().val);
            }
        }

        let mut res = vec![];
        traversal(&root, &mut res);
        res
    }
}
// @lc code=end

