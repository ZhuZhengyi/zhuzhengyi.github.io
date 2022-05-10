/*
 * @lc app=leetcode.cn id=114 lang=cpp
 *
 * [114] 二叉树展开为链表
 *
 * https://leetcode-cn.com/problems/flatten-binary-tree-to-linked-list/description/
 *
 * algorithms
 * Medium (68.83%)
 * Likes:    340
 * Dislikes: 0
 * Total Accepted:    40.5K
 * Total Submissions: 58.9K
 * Testcase Example:  '[1,2,5,3,4,null,6]'
 *
 * 给定一个二叉树，原地将它展开为一个单链表。
 * 
 * 
 * 
 * 例如，给定二叉树
 * 
 * ⁠   1
 * ⁠  / \
 * ⁠ 2   5
 * ⁠/ \   \
 * 3   4   6
 * 
 * 将其展开为：
 * 
 * 1
 * ⁠\
 * ⁠ 2
 * ⁠  \
 * ⁠   3
 * ⁠    \
 * ⁠     4
 * ⁠      \
 * ⁠       5
 * ⁠        \
 * ⁠         6
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
    * 递归
      1. 先分别展开左右子树；
      2. 将root->right指向左子树link头；
      3. 设置一个遍历指针，移动到左子link末尾；
      4. 末尾节点right指针指向右子link；
    */
    void flatten(TreeNode* root) {
        if(!root){
            return;
        }
        flatten(root->left);
        flatten(root->right);
        if(root->left) {
            auto right_link_head = root->right;   //先临时保存右子树链表头指针
            
            root->right = root->left;     //将root->right 指向 left链表
            root->left = NULL;            //root->left 置空

            auto p = root->right;         //以接入right的最后指针
            while(p->right) {
                p = p->right;
            }
            p->right = right_link_head;
        }
    }
};
// @lc code=end

