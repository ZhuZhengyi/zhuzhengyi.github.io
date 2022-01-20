/*
 * @lc app=leetcode.cn id=94 lang=cpp
 *
 * [94] 二叉树的中序遍历
 *
 * https://leetcode-cn.com/problems/binary-tree-inorder-traversal/description/
 *
 * algorithms
 * Easy (75.56%)
 * Likes:    1207
 * Dislikes: 0
 * Total Accepted:    630.9K
 * Total Submissions: 835K
 * Testcase Example:  '[1,null,2,3]'
 *
 * 给定一个二叉树的根节点 root ，返回它的 中序 遍历。
 * 
 * 
 * 
 * 示例 1：
 * 
 * 
 * 输入：root = [1,null,2,3]
 * 输出：[1,3,2]
 * 
 * 
 * 示例 2：
 * 
 * 
 * 输入：root = []
 * 输出：[]
 * 
 * 
 * 示例 3：
 * 
 * 
 * 输入：root = [1]
 * 输出：[1]
 * 
 * 
 * 示例 4：
 * 
 * 
 * 输入：root = [1,2]
 * 输出：[2,1]
 * 
 * 
 * 示例 5：
 * 
 * 
 * 输入：root = [1,null,2]
 * 输出：[1,2]
 * 
 * 
 * 
 * 
 * 提示：
 * 
 * 
 * 树中节点数目在范围 [0, 100] 内
 * -100 
 * 
 * 
 * 
 * 
 * 进阶: 递归算法很简单，你可以通过迭代算法完成吗？
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
    /*
    * ## 递归解法
    * * 1. 先判断退出条件：root == NULL
    * * 2. 先递归调用左子树；
    * * 3. 输出root节点；
    * * 4. 再递归调用右子树；
    * * 
    */
    void helper_rec(TreeNode *root, vector<int> &res) {
        if (!root) {
            return;
        }
        helper_rec(root->left, res);
        res.push_back(root->val);
        helper_rec(root->right, res);
    }

    /*
    * ## 迭代解法
    * * 1. 使用一个栈来保存遍历左子树时的当前节点；
    * * 2. 
    */
    void helper_iter(TreeNode *root, vector<int> &res) {
        stack<TreeNode *> tmp;

        while (root || !tmp.empty()) {
            //依次将每个节点沿左子树遍历入栈
            while(root) {
                tmp.push(root);        
                root = root->left;
            }
            //待节点沿左子树遍历到底后，依次弹出左节点；
            root = tmp.top(); tmp.pop(); 
            res.push_back(root->val);    //输出当前节点
            root = root->right;          //切换到右子树
        }

    }
public:
    vector<int> inorderTraversal(TreeNode* root) {
        vector<int> res ;
        helper_iter(root, res);
        return res;
    }
};
// @lc code=end

