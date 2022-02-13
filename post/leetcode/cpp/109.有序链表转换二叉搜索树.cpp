/*
 * @lc app=leetcode.cn id=109 lang=cpp
 *
 * [109] 有序链表转换二叉搜索树
 *
 * https://leetcode-cn.com/problems/convert-sorted-list-to-binary-search-tree/description/
 *
 * algorithms
 * Medium (76.22%)
 * Likes:    659
 * Dislikes: 0
 * Total Accepted:    105.8K
 * Total Submissions: 138.7K
 * Testcase Example:  '[-10,-3,0,5,9]'
 *
 * 给定一个单链表，其中的元素按升序排序，将其转换为高度平衡的二叉搜索树。
 * 
 * 本题中，一个高度平衡二叉树是指一个二叉树每个节点 的左右两个子树的高度差的绝对值不超过 1。
 * 
 * 示例:
 * 
 * 给定的有序链表： [-10, -3, 0, 5, 9],
 * 
 * 一个可能的答案是：[0, -3, 9, -10, null, 5], 它可以表示下面这个高度平衡二叉搜索树：
 * 
 * ⁠     0
 * ⁠    / \
 * ⁠  -3   9
 * ⁠  /   /
 * ⁠-10  5
 * 
 * 
 */

// @lc code=start
/**
 * Definition for singly-linked list.
 * struct ListNode {
 *     int val;
 *     ListNode *next;
 *     ListNode() : val(0), next(nullptr) {}
 *     ListNode(int x) : val(x), next(nullptr) {}
 *     ListNode(int x, ListNode *next) : val(x), next(next) {}
 * };
 */
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
    * 1. 使用快慢指针查找链表的中间节点；
    * 2. 根据中间节点生成二叉搜索树的根节点；
    * 3. 链表前半部分递归建立左子树；
    * 4. 链表后半部分递归建立右子树；
    */   
    TreeNode* sortedListToBST(ListNode* head) {
        if (!head) return nullptr;
        if (!(head->next)) return new TreeNode(head->val);

        auto p=head, p1=head, p2 = head;
        while (p2&&p2->next) {
            p=p1;
            p1=p1->next;
            p2=p2->next->next;
        }
        p->next = nullptr;

        //auto midNode = split2SubList(head);
        auto root = new TreeNode(p1->val);
        root->left = sortedListToBST(head);
        root->right = sortedListToBST(p1->next);

        return root;
    }
};
// @lc code=end

