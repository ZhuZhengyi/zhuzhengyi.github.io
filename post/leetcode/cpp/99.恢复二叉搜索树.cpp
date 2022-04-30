/*
 * @lc app=leetcode.cn id=99 lang=cpp
 *
 * [99] 恢复二叉搜索树
 *
 * https://leetcode-cn.com/problems/recover-binary-search-tree/description/
 *
 * algorithms
 * Medium (60.48%)
 * Likes:    708
 * Dislikes: 0
 * Total Accepted:    99K
 * Total Submissions: 163.8K
 * Testcase Example:  '[1,3,null,null,2]'
 *
 * 给你二叉搜索树的根节点 root ，该树中的 恰好 两个节点的值被错误地交换。请在不改变其结构的情况下，恢复这棵树 。
 * 
 * 
 * 
 * 示例 1：
 * 
 * 
 * 输入：root = [1,3,null,null,2]
 * 输出：[3,1,null,null,2]
 * 解释：3 不能是 1 的左孩子，因为 3 > 1 。交换 1 和 3 使二叉搜索树有效。
 * 
 * 
 * 示例 2：
 * 
 * 
 * 输入：root = [3,1,4,null,null,2]
 * 输出：[2,1,4,null,null,3]
 * 解释：2 不能在 3 的右子树中，因为 2 < 3 。交换 2 和 3 使二叉搜索树有效。
 * 
 * 
 * 
 * 提示：
 * 
 * 
 * 树上节点的数目在范围 [2, 1000] 内
 * -2^31 <= Node.val <= 2^31 - 1
 * 
 * 
 * 
 * 
 * 进阶：使用 O(n) 空间复杂度的解法很容易实现。你能想出一个只使用 O(1) 空间的解决方案吗？
 * 
 */

struct TreeNode {
    int val;
    TreeNode *left;
    TreeNode *right;
    TreeNode() : val(0), left(nullptr), right(nullptr) {}
    TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
    TreeNode(int x, TreeNode *left, TreeNode *right) : val(x), left(left), right(right) {}
};

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
    TreeNode *prev;  //当前节点前一个节点
    TreeNode *node1; //逆序开始节点
    TreeNode *node2; //逆序最后节点
public:
    /**
     * ## 解题思路
     * 1. 中序遍历二叉搜索树，正常的二叉搜索树为有序递增序列；
     * 2. 乱序的两个节点之间的所有节点为逆序排列；
     * 3. 遍历时，根据当前节点和前一个节点的大小关系，判断是否逆序；
     * 4. 使用两个临时节点变量分别记录逆序的开始和结束；
     * 5. 遍历完成后，交换逆序开始和结束节点；
     */
    void recoverTree(TreeNode* root) {
        inorder(root);
        swap(node1->val, node2->val);
    }

    // 中序遍历二叉树
    void inorder(TreeNode* node) {
        //节点为nil，结束遍历
        if (!node) {
            return;
        }
        //递归中序遍历左子树
        inorder(node->left);

        //处理当前节点

        // 如果当前节点和前一个节点逆序
        if (prev && node->val < prev->val) {
            if (!node1) {       //第一个逆序节点为开始节点
                node1 = prev;   
            } 
            node2 = node;     //逆序结束节点
        }

        //当前节点处理完后，保存当前节点为上一个节点
        prev = node;

        //递归中序遍历右子树
        inorder(node->right);
    }
};
void swap(int &a, int& b) {
    a ^= b;
    b ^= a;
    a ^= b;
}
// @lc code=end

