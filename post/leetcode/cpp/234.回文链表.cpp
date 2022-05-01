/*
 * @lc app=leetcode.cn id=234 lang=cpp
 *
 * [234] 回文链表
 *
 * https://leetcode-cn.com/problems/palindrome-linked-list/description/
 *
 * algorithms
 * Easy (51.35%)
 * Likes:    1365
 * Dislikes: 0
 * Total Accepted:    422.5K
 * Total Submissions: 822.2K
 * Testcase Example:  '[1,2,2,1]'
 *
 * 给你一个单链表的头节点 head ，请你判断该链表是否为回文链表。如果是，返回 true ；否则，返回 false 。
 * 
 * 
 * 
 * 示例 1：
 * 
 * 
 * 输入：head = [1,2,2,1]
 * 输出：true
 * 
 * 
 * 示例 2：
 * 
 * 
 * 输入：head = [1,2]
 * 输出：false
 * 
 * 
 * 
 * 
 * 提示：
 * 
 * 
 * 链表中节点数目在范围[1, 10^5] 内
 * 0 <= Node.val <= 9
 * 
 * 
 * 
 * 
 * 进阶：你能否用 O(n) 时间复杂度和 O(1) 空间复杂度解决此题？
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
class Solution {
public:
    /**
     * ## 解题思路
     * 1. 使用快慢指针，获取链表中间节点指针；
     * 2. 翻转后半链表；
     * 3. 依次比较前半链表后翻转后的后半链表；
    */
    bool isPalindrome(ListNode* head) {
        ListNode* l0 = head; 
        ListNode* l1 = head; 

        //分割链表为前后两个部分
        while(l0->next && l1->next) {
            l0=l0->next;
            l1=l1->next;
            if (l1->next) {
                l1=l1->next;
            }
        }

        //翻转后部链表
        ListNode* l1r_head = nullptr;
        l1 = l0;
        while(l1) {
            l0=l1;
            l1=l1->next;

            l0->next=l1r_head;
            l1r_head=l0;
        }

        //依次对比两个链表各个值
        l0 = head;
        l1 = l1r_head;
        while(l0 && l1) {
            if (l0->val != l1->val) {
                return false;
            }
            l0=l0->next;
            l1=l1->next;
        }

        return true;
    }
};
// @lc code=end

