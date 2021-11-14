/*
 * @lc app=leetcode.cn id=113 lang=rust
 *
 * [113] 路径总和 II
 *
 * https://leetcode-cn.com/problems/path-sum-ii/description/
 *
 * algorithms
 * Medium (62.78%)
 * Likes:    608
 * Dislikes: 0
 * Total Accepted:    187.4K
 * Total Submissions: 298.6K
 * Testcase Example:  '[5,4,8,11,null,13,4,7,2,null,null,5,1]\n22'
 *
 * 给你二叉树的根节点 root 和一个整数目标和 targetSum ，找出所有 从根节点到叶子节点 路径总和等于给定目标和的路径。
 * 
 * 叶子节点 是指没有子节点的节点。
 * 
 * 
 * 
 * 
 * 
 * 示例 1：
 * 
 * 
 * 输入：root = [5,4,8,11,null,13,4,7,2,null,null,5,1], targetSum = 22
 * 输出：[[5,4,11,2],[5,8,4,5]]
 * 
 * 
 * 示例 2：
 * 
 * 
 * 输入：root = [1,2,3], targetSum = 5
 * 输出：[]
 * 
 * 
 * 示例 3：
 * 
 * 
 * 输入：root = [1,2], targetSum = 0
 * 输出：[]
 * 
 * 
 * 
 * 
 * 提示：
 * 
 * 
 * 树中节点总数在范围 [0, 5000] 内
 * -1000 
 * -1000 
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
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    /// ## 解题思路：
    /// 中序递归遍历整颗树的每一个节点`node`：
    /// 1. 遍历过程中，使用一个数组记录`node_path[]`遍历过的节点；
    /// 2. 节点为空，不做任何处理；
    /// 3. 叶子节点，且剩余target刚好为node.val, 则找到一条合法路径，
    ///     - 将当前节点加入到`node_path[]`;
    ///     - 将`node_path[]`加入到结果集`res[]`中；
    ///     - 将当前节点从`node_path[]`中弹出;
    /// 4. 其他情况：
    ///     - 4.1 将当前节点加入到路径中；
    ///     - 4.2 将 target_num -= node.val;
    ///     - 4.3 递归处理左子树;
    ///     - 4.4 递归处理右子树；
    ///     - 4.5 将节点从路径中弹出；
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> Vec<Vec<i32>> {
        fn path_sum_rec(node: &Option<Rc<RefCell<TreeNode>>>, target_sum: i32, node_path: &mut Vec<i32>, res: &mut Vec<Vec<i32>>) {
            match node {
                None => {},
                Some(node) if node.borrow().left.is_none() && node.borrow().right.is_none() && node.borrow().val == target_sum => {
                    node_path.push(node.borrow().val);
                    res.push(node_path.iter().cloned().collect::<Vec<_>>());
                    node_path.pop();
                }
                Some(node) => {
                    node_path.push(node.borrow().val);
                    path_sum_rec(&node.borrow().left, target_sum-node.borrow().val, node_path, res);
                    path_sum_rec(&node.borrow().right, target_sum-node.borrow().val, node_path, res);
                    node_path.pop();
                }
            }
        }

        let mut res: Vec<Vec<i32>> = vec![];
        let mut node_path: Vec<i32> = vec![];
        path_sum_rec(&root, target_sum, &mut node_path, &mut res);

        res
    }
}
// @lc code=end

