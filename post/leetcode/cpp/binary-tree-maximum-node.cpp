/*
@Copyright:LintCode
@Author:   justice_103
@Problem:  http://www.lintcode.com/problem/binary-tree-maximum-node
@Language: C++
@Datetime: 17-03-10 14:15
*/

class Solution {
public:
    /**
     * @param root the root of binary tree
     * @return the max node
     */
    TreeNode* maxNode(TreeNode* root) {
        // Write your code here
        if (root == NULL) {
            return NULL;
        }
        auto max = root;
        if (root->left != NULL) {
            auto maxl = this->maxNode(root->left);
            max = max->val > maxl->val ? max : maxl;
        }
        if (root->right != NULL) {
            auto maxl = this->maxNode(root->right);
            max = max->val > maxl->val ? max : maxl;
        }
        
        return max;
    }
};