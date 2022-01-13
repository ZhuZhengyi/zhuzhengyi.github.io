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
    pub fn heigh(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            None => 0 as i32,
            Some(node) => 1 + cmp::max(heigh(node.borrow().left.clone()), heigh(node.borrow().right.clone())),
        }
    }

    pub fn print_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<String>> {
        let mut res: Vec<Vec<String>> = Vec::new();
        //let mut res_level: Vec = vec![];

        if root.is_none() {
            return res;
        }

        let mut level = Vec::new();
        level.push(root.unwrap().clone());

        let mut level_level = vec![ level ];
        while ! level_level.is_empty() {
            let mut level = level_level.remove(0);
            let mut next_level = Vec::new();
            let mut level_res = Vec::new();
            level.iter().for_each(|node| {
                if ! node.borrow().is_none() {
                    level_res.push(node.borrow().val.to_string());
                    next_level.push( node.borrow().left.clone());
                    next_level.push( node.borrow().right.clone()); 
                } else {
                    level_res.push("".to_string());
                }
            });
            res.push(level_res);
            if ! next_level.is_empty() {
                level_level.push(next_level);
            }
        }

        res
    }
}
// @lc code=end

