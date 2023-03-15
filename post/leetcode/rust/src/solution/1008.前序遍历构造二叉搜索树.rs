/*
 * @lc app=leetcode.cn id=1008 lang=rust
 *
 * [1008] 前序遍历构造二叉搜索树
 *
 * https://leetcode.cn/problems/construct-binary-search-tree-from-preorder-traversal/description/
 *
 * algorithms
 * Medium (72.08%)
 * Likes:    248
 * Dislikes: 0
 * Total Accepted:    28.4K
 * Total Submissions: 39.4K
 * Testcase Example:  '[8,5,1,7,10,12]'
 *
 * 给定一个整数数组，它表示BST(即 二叉搜索树 )的 先序遍历 ，构造树并返回其根。
 *
 * 保证 对于给定的测试用例，总是有可能找到具有给定需求的二叉搜索树。
 *
 * 二叉搜索树 是一棵二叉树，其中每个节点， Node.left 的任何后代的值 严格小于 Node.val , Node.right 的任何后代的值
 * 严格大于 Node.val。
 *
 * 二叉树的 前序遍历 首先显示节点的值，然后遍历Node.left，最后遍历Node.right。
 *
 *
 *
 * 示例 1：
 *
 *
 *
 *
 * 输入：preorder = [8,5,1,7,10,12]
 * 输出：[8,5,10,1,7,null,12]
 *
 *
 * 示例 2:
 *
 *
 * 输入: preorder = [1,3]
 * 输出: [1,null,3]
 *
 *
 *
 *
 * 提示：
 *
 *
 * 1 <= preorder.length <= 100
 * 1 <= preorder[i] <= 10^8
 * preorder 中的值 互不相同
 *
 *
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
use std::rc::Rc;
impl Solution {
    /// ## 解题思路
    /// - 递归
    /// 1. 前序遍历又叫先根遍历, 其顺序为: 根节点->左子树->右子树;
    /// 2. 由于是二叉搜索树,其左子树的所有节点值必然<根节点,而右子树的所有节点值>根节点值;
    /// 3. 所有数组中的首元素为根节点, 而之后所有小于首元素的元素必然属于左子树,
    ///    所有大于首元素的元素属于右子树;
    /// 4. 所以查找数组中第一个>首元素的元素index, 即可将问题分解为更小规模;
    pub fn bst_from_preorder(preorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        match preorder.len() {
            0 => None,
            1 => Some(Rc::new(RefCell::new(TreeNode::new(preorder[0])))),
            _ => {
                if let Some(p) = preorder.iter().position(|&x| x > preorder[0]) {
                    Some(Rc::new(RefCell::new(TreeNode {
                        val: preorder[0],
                        left: Solution::bst_from_preorder(preorder[1..p].to_vec()),
                        right: Solution::bst_from_preorder(preorder[p..].to_vec()),
                    })))
                } else {
                    Some(Rc::new(RefCell::new(TreeNode {
                        val: preorder[0],
                        left: Solution::bst_from_preorder(preorder[1..].to_vec()),
                        right: None,
                    })))
                }
            }
        }
    }
}
// @lc code=end
