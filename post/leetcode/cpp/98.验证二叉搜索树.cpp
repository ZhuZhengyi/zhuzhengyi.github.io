/*
 * @lc app=leetcode.cn id=98 lang=cpp
 *
 * [98] 验证二叉搜索树
 *
 * https://leetcode-cn.com/problems/validate-binary-search-tree/description/
 *
 * algorithms
 * Medium (35.36%)
 * Likes:    1387
 * Dislikes: 0
 * Total Accepted:    407.6K
 * Total Submissions: 1.2M
 * Testcase Example:  '[2,1,3]'
 *
 * 给你一个二叉树的根节点 root ，判断其是否是一个有效的二叉搜索树。
 * 
 * 有效 二叉搜索树定义如下：
 * 
 * 
 * 节点的左子树只包含 小于 当前节点的数。
 * 节点的右子树只包含 大于 当前节点的数。
 * 所有左子树和右子树自身必须也是二叉搜索树。
 * 
 * 
 * 
 * 
 * 示例 1：
 * 
 * 
 * 输入：root = [2,1,3]
 * 输出：true
 * 
 * 
 * 示例 2：
 * 
 * 
 * 输入：root = [5,1,4,null,null,3,6]
 * 输出：false
 * 解释：根节点的值是 5 ，但是右子节点的值是 4 。
 * 
 * 
 * 
 * 
 * 提示：
 * 
 * 
 * 树中节点数目范围在[1, 10^4] 内
 * -2^31 <= Node.val <= 2^31 - 1
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
    /*
    ## 解题思路
    * 递归法：
    *   有效二叉树条件：
    *   1. 空树是一个有效的二叉搜索树；
    *   2. 如果存在左子树，则同时满足以下两个条件：
    *       2.1 根节点val > 左子树的最大节点值；
    *       2.2 左子树也是一颗有效二叉搜索树；
    *   3. 如果存在右子树，同时满足：
    *       3.1 根节点val < 右子树最小节点值；
    *       3.2 右子树也是一颗有效二叉搜索树；
    */
    bool isValidBST(TreeNode* root) {
        return isValid(root, NULL, NULL);
    }

    // 以root为根，lower为下限， upper为上限的树是否为有效二叉树
    bool isValid(TreeNode* root, int* lower, int* upper) {
        if (!root) return true;

        if (upper && root->val >= *upper) return false;
        if (lower && root->val <= *lower) return false;

        return isValid(root->left, lower, &(root->val))
            && isValid(root->right, &(root->val), upper);

    }
};
// @lc code=end

