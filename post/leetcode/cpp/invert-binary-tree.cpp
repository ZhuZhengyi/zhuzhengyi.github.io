/*
@Copyright:LintCode
@Author:   justice_103
@Problem:  http://www.lintcode.com/problem/invert-binary-tree
@Language: C++
@Datetime: 15-07-11 23:45
*/

/**
 * Definition of TreeNode:
 * class TreeNode {
 * public:
 *     int val;
 *     TreeNode *left, *right;
 *     TreeNode(int val) {
 *         this->val = val;
 *         this->left = this->right = NULL;
 *     }
 * }
 */
class Solution {
public:
    /**
     * @param root: a TreeNode, the root of the binary tree
     * @return: nothing
     */
    void invertBinaryTree(TreeNode *root) {
        // write your code here
        //assert(root!=NULL);
        
        std::queue<TreeNode*> q;
        q.push(root);
        
        while(!q.empty()){
            TreeNode* r=q.front(); q.pop();
            
            std::swap(r->left, r->right);
            
            if(r->left) q.push(r->left);
            if(r->right) q.push(r->right);
        }
    }
};
