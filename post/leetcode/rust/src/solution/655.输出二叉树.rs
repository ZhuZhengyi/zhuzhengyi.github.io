/*
 * @lc app=leetcode.cn id=655 lang=rust
 *
 * [655] 输出二叉树
 *
 * https://leetcode-cn.com/problems/print-binary-tree/description/
 *
 * algorithms
 * Medium (60.01%)
 * Likes:    113
 * Dislikes: 0
 * Total Accepted:    10.2K
 * Total Submissions: 17K
 * Testcase Example:  '[1,2]'
 *
 * 在一个 m*n 的二维字符串数组中输出二叉树，并遵守以下规则：
 *
 *
 * 行数 m 应当等于给定二叉树的高度。
 * 列数 n 应当总是奇数。
 *
 * 根节点的值（以字符串格式给出）应当放在可放置的第一行正中间。根节点所在的行与列会将剩余空间划分为两部分（左下部分和右下部分）。你应该将左子树输出在左下部分，右子树输出在右下部分。左下和右下部分应当有相同的大小。即使一个子树为空而另一个非空，你不需要为空的子树输出任何东西，但仍需要为另一个子树留出足够的空间。然而，如果两个子树都为空则不需要为它们留出任何空间。
 * 每个未使用的空间应包含一个空的字符串""。
 * 使用相同的规则输出子树。
 *
 *
 * 示例 1:
 *
 *
 * 输入:
 * ⁠    1
 * ⁠   /
 * ⁠  2
 * 输出:
 * [["", "1", ""],
 * ⁠["2", "", ""]]
 *
 *
 * 示例 2:
 *
 *
 * 输入:
 * ⁠    1
 * ⁠   / \
 * ⁠  2   3
 * ⁠   \
 * ⁠    4
 * 输出:
 * [["", "", "", "1", "", "", ""],
 * ⁠["", "2", "", "", "", "3", ""],
 * ⁠["", "", "4", "", "", "", ""]]
 *
 *
 * 示例 3:
 *
 *
 * 输入:
 * ⁠     1
 * ⁠    / \
 * ⁠   2   5
 * ⁠  /
 * ⁠ 3
 * ⁠/
 * 4
 * 输出:
 * [["",  "",  "", "",  "", "", "", "1", "",  "",  "",  "",  "", "", ""]
 * ⁠["",  "",  "", "2", "", "", "", "",  "",  "",  "",  "5", "", "", ""]
 * ⁠["",  "3", "", "",  "", "", "", "",  "",  "",  "",  "",  "", "", ""]
 * ⁠["4", "",  "", "",  "", "", "", "",  "",  "",  "",  "",  "", "", ""]]
 *
 *
 * 注意: 二叉树的高度在范围 [1, 10] 中。
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
use std::collections::VecDeque;
use std::rc::Rc;
impl Solution {

    /// ## 解题思路
    /// - 队列
    /// 1. 首先计算树的最大深度;
    /// 2. 根据最大深度,计算打印数组的长,宽, 根据长宽初始化打印数组;
    /// 3. 根节点在数组中的位置为(0, width / 2), 将(root, r=0, left=0, right=width-1)四元组入队列;
    /// 4. 依次取出队列中的四元组, 根据四元组值更新打印数组;
    /// 5. 如果当前节点存在左子节点,右子节点加入到队列尾部;
    pub fn print_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<String>> {
        /// 获取二叉树的高度
        fn get_heigh(root: Option<Rc<RefCell<TreeNode>>>) -> usize {
            match root {
                None => 0,
                Some(node) => {
                    1 + std::cmp::max(
                        get_heigh(node.borrow().left.clone()),
                        get_heigh(node.borrow().right.clone()),
                        )
                }
            }
        }
        let heigh = get_heigh(root.clone());
        let width = ((1 << heigh as u32) - 1) as usize;
        let mut res = vec![vec!["".to_string(); width]; heigh];

        if root.is_none() {
            return res;
        }

        let mut q = VecDeque::new();
        q.push_back((root.unwrap().clone(), 0, 0, width-1)); //将根节点push到队列尾部;
        // 依次从队列头取出一个节点元组
        while let Some((node, row, left, right)) = q.pop_front() {
            let col = (left + right) / 2; //计算当前节点所在的col
            res[row][col] = node.borrow().val.to_string(); //更新打印数组当前节点值

            // 如果存在左子节点
            if let Some(left_node) = node.borrow().left.as_ref() {
                q.push_back((left_node.clone(), row+1, left, col-1)); //将左子节点push到队列尾
            }
            // 如果存在右子节点
            if let Some(right_node) = node.borrow().right.as_ref() {
                q.push_back((right_node.clone(), row+1, col+1, right)); //将右子节点push到队列尾
            }
        }
        
        res
    }
}
// @lc code=end
