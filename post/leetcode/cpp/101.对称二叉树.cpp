/*
 * @lc app=leetcode.cn id=101 lang=cpp
 *
 * [101] 对称二叉树
 *
 * https://leetcode-cn.com/problems/symmetric-tree/description/
 *
 * algorithms
 * Easy (56.66%)
 * Likes:    1665
 * Dislikes: 0
 * Total Accepted:    459.5K
 * Total Submissions: 810.7K
 * Testcase Example:  '[1,2,2,3,4,4,3]'
 *
 * 给定一个二叉树，检查它是否是镜像对称的。
 * 
 * 
 * 
 * 例如，二叉树 [1,2,2,3,4,4,3] 是对称的。
 * 
 * ⁠   1
 * ⁠  / \
 * ⁠ 2   2
 * ⁠/ \ / \
 * 3  4 4  3
 * 
 * 
 * 
 * 
 * 但是下面这个 [1,2,2,null,3,null,3] 则不是镜像对称的:
 * 
 * ⁠   1
 * ⁠  / \
 * ⁠ 2   2
 * ⁠  \   \
 * ⁠  3    3
 * 
 * 
 * 
 * 
 * 进阶：
 * 
 * 你可以运用递归和迭代两种方法解决这个问题吗？
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
    * * 解法一：递归法
    * * 转化为镜像树问题；
    * * 镜像子树：根节点相等 且 相互的左子树，右子树互为镜像子树；
    */
    bool isSymmetric(TreeNode* root) {
        return isMirrorByRec(root, root);
        //return isMirrorByIter(root, root);
    }

    // check p,q is mirror
    bool isMirrorByRec(TreeNode* p, TreeNode* q) {
        if (!p && !q) return true;
        if (!p || !q) return false;
        return (p->val == q->val 
            && isMirrorByRec(p->left, q->right)
            && isMirrorByRec(p->right, q->left)
        ) ;
    }
    /*
    ## 解法二：迭代法
       * 使用一个队列层历该树；
       * 初始将root入队两次；
       * 然后每次队列出队时，
    */
    bool isMirrorByIter(TreeNode* p, TreeNode* q) {
        if (!p && !q) return true;
        if (!p || !q) return false;

        queue<TreeNode*> queue;
        queue.push(p);
        queue.push(q);
        while(!queue.empty()) {
            p = queue.front(); queue.pop();
            q = queue.front(); queue.pop();
            if (!p && !q) continue;
            if (!p || !q) return false;
            if (p->val != q->val) return false;
            queue.push(p->left);
            queue.push(q->right);
            queue.push(p->right);
            queue.push(q->left);
        }

        return true;
    }

};
// @lc code=end

