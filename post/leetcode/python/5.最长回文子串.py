#
# @lc app=leetcode.cn id=5 lang=python3
#
# [5] 最长回文子串
#
# https://leetcode-cn.com/problems/longest-palindromic-substring/description/
#
# algorithms
# Medium (27.66%)
# Likes:    1487
# Dislikes: 0
# Total Accepted:    144.2K
# Total Submissions: 521.2K
# Testcase Example:  '"babad"'
#
# 给定一个字符串 s，找到 s 中最长的回文子串。你可以假设 s 的最大长度为 1000。
# 
# 示例 1：
# 
# 输入: "babad"
# 输出: "bab"
# 注意: "aba" 也是一个有效答案。
# 
# 示例 2：
# 
# 输入: "cbbd"
# 输出: "bb"
# 
# @lc code=start
class Solution:
    def longestPalindrome(self, s: str) -> str:
        '''
        ## 解题思路
        dp[l, r]: 表示s[l:r+1] 是否为回文串
        状态转移方程：
        dp[l, r] = (s[l] == s[r] and (r - l <= 2 or dp[l + 1, r - 1]))

        '''
        size = len(s)
        if size < 2:
            return s
        dp = [ [ False for _ in range(size) ] for _ in range(size) ]
        longest = 1
        res = s[0]
        for r in range(1, size):
            for l in range(r):
                if s[l] == s[r] and ( r -l <=2 or dp[l+1][r-1] ):
                    dp[l][r] = True
                    cur_len = r - l + 1
                    if cur_len > longest:
                        longest = cur_len
                        res = s[l:r+1]
                        
        return res
        
# @lc code=end

