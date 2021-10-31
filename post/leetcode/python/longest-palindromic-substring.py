# coding:utf-8
'''
@Copyright:LintCode
@Author:   justice_103
@Problem:  http://www.lintcode.com/problem/longest-palindromic-substring
@Language: Python
@Datetime: 17-03-18 23:09
'''

class Solution:
    # @param {string} s input string
    # @return {string} the longest palindromic substring
    def longestPalindrome(self, s):
        # Write your code here
        max_len = 0
        n = len(s)
        pal = [[ None ] * n for _ in range(n)]
        a=None
        i=0
        while i < len(s):
            j = i
            while j>=0:
                if s[i] == s[j] and ( i-j<2 or pal[j+1][i-1]) :
                    pal[j][i] = True
                    if i - j + 1 > max_len:
                        a=s[j:i+1]
                        max_len = i-j +1
                else:
                    pal[j][i] = False
                j -= 1
            i += 1
        return a