/*
@Copyright:LintCode
@Author:   justice_103
@Problem:  http://www.lintcode.com/problem/binary-tree-path-sum
@Language: C++
@Datetime: 17-03-12 09:14
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
     * @param root the root of binary tree
     * @param target an integer
     * @return all valid paths
     */
    vector<vector<int>> binaryTreePathSum(TreeNode *root, int target) {
        // Write your code here
        vector<vector<int>> result;
        vector<int> path;
        
        this->search(root, target, path, result);
        return result;
    }
    
    void search(TreeNode *root, int target, vector<int>& path, vector<vector<int>>& result) {
        if (root == NULL) {
            return;
        }
        path.push_back(root->val);
        if (root->left == NULL && root->right == NULL && root->val == target) {
            result.push_back(path);
        }
        if (root->left != NULL) {
            this->search(root->left, target-root->val, path, result);
        }
        if (root->right != NULL) {
            this->search(root->right, target-root->val, path, result);
        }
        path.pop_back();
        
    }
    
    
};