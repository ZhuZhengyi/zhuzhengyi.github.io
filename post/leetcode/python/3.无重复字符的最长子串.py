#
# @lc app=leetcode.cn id=3 lang=python3
#
# [3] 无重复字符的最长子串
#
# https://leetcode-cn.com/problems/longest-substring-without-repeating-characters/description/
#
# algorithms
# Medium (32.05%)
# Likes:    2749
# Dislikes: 0
# Total Accepted:    278.1K
# Total Submissions: 867.6K
# Testcase Example:  '"abcabcbb"'
#
# 给定一个字符串，请你找出其中不含有重复字符的 最长子串 的长度。
#
# 示例 1:
#
# 输入: "abcabcbb"
# 输出: 3
# 解释: 因为无重复字符的最长子串是 "abc"，所以其长度为 3。
#
#
# 示例 2:
#
# 输入: "bbbbb"
# 输出: 1
# 解释: 因为无重复字符的最长子串是 "b"，所以其长度为 1。
#
#
# 示例 3:
#
# 输入: "pwwkew"
# 输出: 3
# 解释: 因为无重复字符的最长子串是 "wke"，所以其长度为 3。
# 请注意，你的答案必须是 子串 的长度，"pwke" 是一个子序列，不是子串。
#
#
#

# @lc code=start
class Solution:
    '''
    ## 解题思路
    * 双指针法
    * 1. 按序遍历字符串；
    * 2. 
    '''
    def lengthOfLongestSubstring(self, s: str) -> int:
        recorded = {}  # 已经出现的字符的最后一次出现的index
        l, r = 0, 0    # 
        m = r - l      # max len for tmp substring
        for c in s:  #遍历字符串字符
            # 如果当前字符出现过，并且在窗口内(>=l)
            if c in recorded and recorded[c] >= l:
                l = recorded[c] + 1 #则收缩左指针，从而维持窗口内没有重复的字符 
            else:  # 否则，之前的窗口不包含当前字符
                m = max(m, r-l+1)   #则通过计算当前窗口值获得当前最大不重复字符串的值
            recorded[c] = r   #更新当前字符最新的index
            r += 1              #扩大窗口右边界

        return m
# @lc code=end

