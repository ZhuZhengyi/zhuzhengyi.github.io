/*
 * @lc app=leetcode.cn id=5 lang=cpp
 *
 * [5] 最长回文子串
 *
 * https://leetcode-cn.com/problems/longest-palindromic-substring/description/
 *
 * algorithms
 * Medium (36.36%)
 * Likes:    5031
 * Dislikes: 0
 * Total Accepted:    975.8K
 * Total Submissions: 2.7M
 * Testcase Example:  '"babad"'
 *
 * 给你一个字符串 s，找到 s 中最长的回文子串。
 * 
 * 
 * 
 * 示例 1：
 * 
 * 
 * 输入：s = "babad"
 * 输出："bab"
 * 解释："aba" 同样是符合题意的答案。
 * 
 * 
 * 示例 2：
 * 
 * 
 * 输入：s = "cbbd"
 * 输出："bb"
 * 
 * 
 * 
 * 
 * 提示：
 * 
 * 
 * 1 <= s.length <= 1000
 * s 仅由数字和英文字母组成
 * 
 * 
 */

#include <string>
#include <vector>

using namespace std;

// @lc code=start
class Solution {
public:
    /**
    * ## 解题思路
    * 动态规划
    1. 定义 dp[i][j]: s[i:j]是否为回文子串；
    2. 递推公式：
        dp[i][j] = (s[i]==s[j] && dp[i+1][j-1])
    3. 初始条件：
        dp[i][i] = true
    4. 注意遍历秩序
    */
    string longestPalindrome(string s) {
        int n = s.size();
        vector<vector<bool> > dp(n, vector<bool>(n, false));
        int start = 0;
        int longest = 1;
        //双指针依次遍历
        for(int r=1; r<n; r++) {
            dp[r][r] = true;
            for(int l=0; l<r; l++) {
                dp[l][l] = true;
                if (s[l] == s[r] && (r-l<=2 || dp[l+1][r-1])) {
                    dp[l][r] = true;
                    if (r-l+1>longest) {
                        longest = r-l+1;
                        start = l;
                    }
                }
            }
        }

        return s.substr(start, longest);
    }
};
// @lc code=end

