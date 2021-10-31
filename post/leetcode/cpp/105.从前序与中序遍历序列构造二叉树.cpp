/*
 * @lc app=leetcode.cn id=105 lang=cpp
 *
 * [105] 从前序与中序遍历序列构造二叉树
 *
 * https://leetcode-cn.com/problems/construct-binary-tree-from-preorder-and-inorder-traversal/description/
 *
 * algorithms
 * Medium (65.35%)
 * Likes:    436
 * Dislikes: 0
 * Total Accepted:    65.2K
 * Total Submissions: 99.7K
 * Testcase Example:  '[3,9,20,15,7]\n[9,3,15,20,7]'
 *
 * 根据一棵树的前序遍历与中序遍历构造二叉树。
 * 
 * 注意:
 * 你可以假设树中没有重复的元素。
 * 
 * 例如，给出
 * 
 * 前序遍历 preorder = [3,9,20,15,7]
 * 中序遍历 inorder = [9,3,15,20,7]
 * 
 * 返回如下的二叉树：
 * 
 * ⁠   3
 * ⁠  / \
 * ⁠ 9  20
 * ⁠   /  \
 * ⁠  15   7
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
class Solution {
public:
    TreeNode* buildTree(vector<int>& preorder, vector<int>& inorder) {
       return createTree(preorder, inorder, 0, preorder.size()-1, 0, inorder.size()-1);
    }

    TreeNode* createTree(vector<int>& preorder, vector<int>& inorder, int ps, int pe, int is, int ie){
        if(ps > pe) {
            return nullptr;
        }
        auto node = new TreeNode(preorder[ps]);
        int pos;
        for(int i=is; i<=ie; i++) {
            if(inorder[i] == node->val) {
                pos = i;
                break;
            }
        }

        node->left = createTree(preorder, inorder, ps+1, ps+pos-is, is, pos-1);
        node->right = createTree(preorder, inorder, pe-ie+pos+1, pe, pos+1, ie);
        return node;
    }
};
// @lc code=end

