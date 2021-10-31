# coding:utf-8
'''
@Copyright:LintCode
@Author:   justice_103
@Problem:  http://www.lintcode.com/problem/length-of-last-word
@Language: Python
@Datetime: 15-08-02 13:54
'''

class Solution:
    # @param {string} s A string
    # @return {int} the length of last word
    def lengthOfLastWord(self, s):
        # Write your code here
        n=len(s)
        
        if n<1:
            return 0
        
        i=n-1
        while i>=0 and s[i].isspace():
            i-=1
        j=i
        while j>=0 and s[j].isalnum():
            j-=1
        
        return i-j