/*
 * @lc app=leetcode.cn id=32 lang=cpp
 *
 * [32] 最长有效括号
 *
 * https://leetcode-cn.com/problems/longest-valid-parentheses/description/
 *
 * algorithms
 * Hard (29.67%)
 * Likes:    543
 * Dislikes: 0
 * Total Accepted:    44.1K
 * Total Submissions: 147.6K
 * Testcase Example:  '"(()"'
 *
 * 给定一个只包含 '(' 和 ')' 的字符串，找出最长的包含有效括号的子串的长度。
 * 
 * 示例 1:
 * 
 * 输入: "(()"
 * 输出: 2
 * 解释: 最长有效括号子串为 "()"
 * 
 * 
 * 示例 2:
 * 
 * 输入: ")()())"
 * 输出: 4
 * 解释: 最长有效括号子串为 "()()"
 * 
 * 
 */

// @lc code=start
class Solution {
public:
    /*
    * ## 解题思路
    * 
    */
    int longestValidParentheses(string s) {
        int cnt = 0; //未配对的'('数 
        vector<int> dp(s.size()+1, 0); //以s[i]为尾的最长有效括号长度

        for(size_t i=1; i<=s.size(); i++) {
            if( s[i-1] == '(' ) { //发现'('
                cnt++;
            } else {            //发现')'
                if(cnt>0) {     //存在未配对的'('
                    cnt--;      //减小配对数
                    dp[i] = 2;  //"()"
                    if ( s[i - 2] == ')' ){ //"((***))"
                        dp[i] += dp[i - 1];
                    }
                    dp[i] += dp[i - dp[i]]; //"****(****)"
                }
            }
        }

        return *max_element(dp.begin(), dp.end());
    }
};
// @lc code=end

