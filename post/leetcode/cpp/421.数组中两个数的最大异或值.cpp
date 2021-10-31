/*
 * @lc app=leetcode.cn id=421 lang=cpp
 *
 * [421] 数组中两个数的最大异或值
 *
 * https://leetcode-cn.com/problems/maximum-xor-of-two-numbers-in-an-array/description/
 *
 * algorithms
 * Medium (59.08%)
 * Likes:    128
 * Dislikes: 0
 * Total Accepted:    4.5K
 * Total Submissions: 7.6K
 * Testcase Example:  '[3,10,5,25,2,8]'
 *
 * 给定一个非空数组，数组中元素为 a0, a1, a2, … , an-1，其中 0 ≤ ai < 2^31 。
 * 
 * 找到 ai 和aj 最大的异或 (XOR) 运算结果，其中0 ≤ i,  j < n 。
 * 
 * 你能在O(n)的时间解决这个问题吗？
 * 
 * 示例:
 * 
 * 
 * 输入: [3, 10, 5, 25, 2, 8]
 * 
 * 输出: 28
 * 
 * 解释: 最大的结果是 5 ^ 25 = 28.
 * 
 * 
 */

// @lc code=start
class Solution {
public:
    int findMaximumXOR(vector<int>& nums) {
        TrieNode* head = new TrieNode();
        for (int i=0; i<nums.size(); i++) {
            insert(head, nums[i]);
        }
        int res = find(head, nums);

        return res == INT_MIN ? 0 : res;
   }

   struct TrieNode{
       TrieNode *one;
       TrieNode *zero;
   };
   
   void insert(TrieNode *cur, int n){
       for(int i=31; i>=0; i--) {
           int b=(n>>i)&1;
           if (b) {
               if(!cur->one) {
                   cur->one = new TrieNode();
               }
               cur = cur->one;
           } else {
               if(!cur->zero) {
                   cur->zero = new TrieNode();
               }
               cur = cur->zero;
           }
       }
   }

   int find(TrieNode *head, vector<int>& nums) {
       int res = INT_MIN;
       for(int i = 0; i < nums.size(); i++) {
           TrieNode* tmp = head;
           int cur = 0;
           for(int j = 31; j>=0; j--) {
               int b = (nums[i]>>j)&1;
               if(b) { //xor
                   if(tmp->zero) {
                       cur += pow(2,j);
                       tmp = tmp->zero;
                   } else {
                       tmp = tmp->one;
                   }
               } else {
                   if (tmp->one) {
                       cur += pow(2,j);
                       tmp = tmp->one;
                   } else {
                       tmp = tmp->zero;
                   }
               }
           }
           if (res < cur) {
               res = cur;
           }
       }
       return res;
   }
};
// @lc code=end

