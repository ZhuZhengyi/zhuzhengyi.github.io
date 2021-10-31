# @lc app=leetcode.cn id=14 lang=python3
#
# [14] 最长公共前缀
#
# https://leetcode-cn.com/problems/longest-common-prefix/description/
#
# algorithms
# Easy (35.38%)
# Likes:    783
# Dislikes: 0
# Total Accepted:    153.6K
# Total Submissions: 433.8K
# Testcase Example:  '["flower","flow","flight"]'
#
# 编写一个函数来查找字符串数组中的最长公共前缀。
# 
# 如果不存在公共前缀，返回空字符串 ""。
# 
# 示例 1:
# 
# 输入: ["flower","flow","flight"]
# 输出: "fl"
# 
# 
# 示例 2:
# 
# 输入: ["dog","racecar","car"]
# 输出: ""
# 解释: 输入不存在公共前缀。
# 
# 
# 说明:
# 
# 所有输入只包含小写字母 a-z 。
# 
#

# @lc code=start
class Solution:
    def longestCommonPrefix(self, strs):
        size = len(strs)
        if size == 0:
            return ""
        if size == 1:
            return strs[0]
        min_len = len(min(strs, key=len))
        for i in range(min_len):
            c = strs[0][i]
            for j in range(1, size):
                s = strs[j]
                if c != s[i]:
                    return s[:i]
        if min_len < 1:
            return ""
        return strs[0][:min_len]
    
# @lc code=end

