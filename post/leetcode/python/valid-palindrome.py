# coding:utf-8
'''
@Copyright:LintCode
@Author:   justice_103
@Problem:  http://www.lintcode.com/problem/valid-palindrome
@Language: Python
@Datetime: 15-08-02 08:03
'''

class Solution:
    # @param {string} s A string
    # @return {boolean} Whether the string is a valid palindrome
    def isPalindrome(self, s):
        # Write your code here
        n=len(s)
        
        if n<2:
            return True
        
        i,j=0,n-1
        while i<j:
            while i<n and not s[i].isalnum():
                i+=1
            while j>=0 and not s[j].isalnum():
                j-=1
            if i<j: 
                if s[i].upper()!=s[j].upper():
                    return False
                else:
                    i+=1
                    j-=1
        
        return True
