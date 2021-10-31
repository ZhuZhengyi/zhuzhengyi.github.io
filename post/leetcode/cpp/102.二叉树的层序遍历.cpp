/*
 * @lc app=leetcode.cn id=102 lang=cpp
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
 * 给定一个二叉树，返回其按层次遍历的节点值。
 * （即逐层地，从左到右访问所有节点）。
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

// @lc code=start
/**
 * Definition for a binary tree node.
 * struct TreeNode {
 *     int val;
 *     TreeNode *left;
 *     TreeNode *right;
 *     TreeNode(int x) : val(x), left(NULL), right(NULL) {}
 * };
 */
#include <vector>

class Solution {
public:
  vector<vector<int>> levelOrder(TreeNode *root) {
    vector<vector<int>> res;
    if (!root) {
      return res;
    }
    queue<TreeNode *> q; // queue for treenode which vist with inorder
    // init queue with root
    q.push(root);
    q.push(NULL);          // level flag
    vector<int> cur_level; //
    while (!q.empty()) {
      // deque
      auto n = q.front();
      q.pop();

      if (n) {
        cur_level.push_back(n->val);
        if (n->left) {
          q.push(n->left);
        }
        if (n->right) {
          q.push(n->right);
        }
      } else {
        res.push_back(cur_level);
        cur_level.resize(0);
        if (q.size() > 0) {
          q.push(NULL);
        }
      }
    }

    return res;
  }
};
// @lc code=end
