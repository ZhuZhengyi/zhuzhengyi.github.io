/*
 * @lc app=leetcode.cn id=141 lang=cpp
 *
 * [141] 环形链表
 *
 * https://leetcode-cn.com/problems/linked-list-cycle/description/
 *
 * algorithms
 * Easy (51.39%)
 * Likes:    1476
 * Dislikes: 0
 * Total Accepted:    722K
 * Total Submissions: 1.4M
 * Testcase Example:  '[3,2,0,-4]\n1'
 *
 * 给你一个链表的头节点 head ，判断链表中是否有环。
 * 
 * 如果链表中有某个节点，可以通过连续跟踪 next 指针再次到达，则链表中存在环。 为了表示给定链表中的环，评测系统内部使用整数 pos
 * 来表示链表尾连接到链表中的位置（索引从 0 开始）。注意：pos 不作为参数进行传递 。仅仅是为了标识链表的实际情况。
 * 
 * 如果链表中存在环 ，则返回 true 。 否则，返回 false 。
 * 
 * 
 * 
 * 示例 1：
 * 
 * 
 * 
 * 
 * 输入：head = [3,2,0,-4], pos = 1
 * 输出：true
 * 解释：链表中有一个环，其尾部连接到第二个节点。
 * 
 * 
 * 示例 2：
 * 
 * 
 * 
 * 
 * 输入：head = [1,2], pos = 0
 * 输出：true
 * 解释：链表中有一个环，其尾部连接到第一个节点。
 * 
 * 
 * 示例 3：
 * 
 * 
 * 
 * 
 * 输入：head = [1], pos = -1
 * 输出：false
 * 解释：链表中没有环。
 * 
 * 
 * 
 * 
 * 提示：
 * 
 * 
 * 链表中节点的数目范围是 [0, 10^4]
 * -10^5 <= Node.val <= 10^5
 * pos 为 -1 或者链表中的一个 有效索引 。
 * 
 * 
 * 
 * 
 * 进阶：你能用 O(1)（即，常量）内存解决此问题吗？
 * 
 */

// @lc code=start
/**
 * Definition for singly-linked list.
 * struct ListNode {
 *     int val;
 *     ListNode *next;
 *     ListNode(int x) : val(x), next(NULL) {}
 * };
 */
class Solution {
public:
    /**
    * ## 解题思路
    * * 快慢指针
    * 1. 如果head==nullptr，则链表不存在环形；
    * 2. 使用两个指针l1,l2同时遍历链表;
    * 3. 遍历时l1每次移动一个节点，l2每次移动2个节点；
    * 4. 如果存在l1==l2，则存在环形，退出；
    * 5. 如果l1或l2 == null, 则链表存在终点，无环，退出；
    */
    bool hasCycle(ListNode *head) {
        if (!head) {
            return false;
        }
        ListNode* l1 = head;
        ListNode* l2 = head;
        while(l1->next && l2->next) {
            l1=l1->next;
            l2=l2->next;
            if (l2->next) {
                l2=l2->next;
                if (l1==l2) {
                    return true;
                }
            } else {
                return false;
            }
        }

        return false;
    }
};
// @lc code=end

