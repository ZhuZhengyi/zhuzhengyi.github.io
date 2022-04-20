/*
 * @lc app=leetcode.cn id=687 lang=cpp
 *
 * [687] 最长同值路径
 *
 * https://leetcode-cn.com/problems/longest-univalue-path/description/
 *
 * algorithms
 * Medium (44.69%)
 * Likes:    550
 * Dislikes: 0
 * Total Accepted:    44.3K
 * Total Submissions: 98.8K
 * Testcase Example:  '[5,4,5,1,1,5]'
 *
 * 给定一个二叉树的 root ，返回 最长的路径的长度 ，这个路径中的 每个节点具有相同值 。 这条路径可以经过也可以不经过根节点。
 * 
 * 两个节点之间的路径长度 由它们之间的边数表示。
 * 
 * 
 * 
 * 示例 1:
 * 
 * 
 * 
 * 
 * 输入：root = [5,4,5,1,1,5]
 * 输出：2
 * 
 * 
 * 示例 2:
 * 
 * 
 * 
 * 
 * 输入：root = [1,4,5,4,4,5]
 * 输出：2
 * 
 * 
 * 
 * 
 * 提示:
 * 
 * 
 * 树的节点数的范围是 [0, 10^4] 
 * -1000 <= Node.val <= 1000
 * 树的深度将不超过 1000 
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
 *     TreeNode() : val(0), left(nullptr), right(nullptr) {}
 *     TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
 *     TreeNode(int x, TreeNode *left, TreeNode *right) : val(x), left(left), right(right) {}
 * };
 */
class Solution {
public:
    /**
     * @brief 解题思路
     * @param 递归法
     * 
     */
    int longestUnivaluePath(TreeNode* root) {
        //TODO
        int longestPath = 0;
        if (!root) {
            return 0;
        }

        dfs(root, -1, longestPath);

        return longestPath;
    }

    /**
     * @brief 深度遍历二叉树, 计算以node节点为一个端点的最长同值路径的路径长度
     * @param[in] node: 
     * @param[in] parentVal:
     * @param[context] longestPath: 
     * @param[out] 
     */
    int dfs(TreeNode* node, int parentVal, int& longestPath) {
        if (!node) {
            return 0;
        }

        // 分别计算与当前节点同值的左子树和右子树最长路径长度

        // 以当前节点为端点的左子树的同值路径长度
        int leftPath = dfs(node->left, node->val, longestPath);
        // 以当前节点为端点的右子树的同值路径长度
        int rightPath = dfs(node->right, node->val, longestPath);

        // 经过当前节点的最长路径长度
        if (leftPath + rightPath > longestPath) {
            longestPath = leftPath + rightPath;
        }
        // 如果当前节点和父节点同值，则到父节点的同值长度为：
        // 左右子树最长同值路径中更长的一个 +1
        if (node->val == parentVal) {
            return leftPath>rightPath?leftPath+1:rightPath+1;
        } else { //当前节点和父节点不同值
            return 0;
        }

        return 0;
    }
};
// @lc code=end

